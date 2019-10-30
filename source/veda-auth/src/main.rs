#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use chrono::Utc;
use ini::Ini;
use nng::{Message, Protocol, Socket};
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_json::json;
use serde_json::value::Value as JSONValue;
use uuid::*;
use v_api::{IndvOp, ResultCode};
use v_module::module::{init_log, Module};
use v_onto::datatype::Lang;
use v_onto::individual::Individual;
use v_search::FTQuery;

struct Ticket {
    id: String,
    /// Uri пользователя
    user_uri: String,
    /// login пользователя
    user_login: String,
    /// Код результата, если тикет не валидный != ResultCode.Ok
    result: ResultCode,
    /// Дата начала действия тикета
    start_time: i64,
    /// Дата окончания действия тикета
    end_time: i64,
}

const EMPTY_SHA256_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn main() -> std::io::Result<()> {
    init_log();

    let conf = Ini::load_from_file("veda.properties").expect("fail load veda.properties file");
    let section = conf.section(None::<String>).expect("fail parse veda.properties");

    let ro_storage_url = section.get("auth_url").expect("param [auth_url] not found in veda.properties");

    let server = Socket::new(Protocol::Rep0)?;
    if let Err(e) = server.listen(&ro_storage_url) {
        error!("fail listen, {:?}", e);
        return Ok(());
    }

    let mut module = Module::default();

    let systicket = if let Ok(t) = module.get_sys_ticket_id() {
        t
    } else {
        error!("fail get systicket");
        return Ok(());
    };

    let pass_lifetime = get_password_lifetime(&mut module);

    loop {
        if let Ok(recv_msg) = server.recv() {
            let res = req_prepare(&recv_msg, &systicket, &mut module, pass_lifetime);
            if let Err(e) = server.send(res) {
                error!("fail send {:?}", e);
            }
        }
    }
}

fn req_prepare(request: &Message, systicket: &str, module: &mut Module, pass_lifetime: Option<i64>) -> Message {
    let v: JSONValue = if let Ok(v) = serde_json::from_slice(request.as_slice()) {
        v
    } else {
        JSONValue::Null
    };

    match v["function"].as_str().unwrap_or_default() {
        "authenticate" => {
            let ticket = authenticate(v["login"].as_str(), v["password"].as_str(), v["secret"].as_str(), systicket, module, pass_lifetime.unwrap_or_default());

            let mut res = JSONValue::default();
            res["type"] = json!("ticket");
            res["id"] = json!(ticket.id);
            res["user_uri"] = json!(ticket.user_uri);
            res["user_login"] = json!(ticket.user_login);
            res["result"] = json!(ticket.result as i64);
            res["end_time"] = json!(ticket.end_time);

            return Message::from(res.to_string().as_bytes());
        }
        "get_ticket_trusted" => {}
        _ => {}
    }

    Message::default()
}

fn authenticate(login: Option<&str>, password: Option<&str>, secret: Option<&str>, systicket: &str, module: &mut Module, pass_lifetime: i64) -> Ticket {
    let mut ticket = Ticket {
        id: String::default(),
        user_uri: String::default(),
        user_login: String::default(),
        result: ResultCode::AuthenticationFailed,
        start_time: 0,
        end_time: 0,
    };

    info!("authenticate, login={:?} password={:?}, secret={:?}", login, password, secret);

    let login = login.unwrap_or_default();
    let password = password.unwrap_or_default();
    let secret = secret.unwrap_or_default();

    if login.is_empty() || login.len() < 3 {
        return ticket;
    }

    if !secret.is_empty() && secret.len() > 5 && password == EMPTY_SHA256_HASH {
        ticket.result = ResultCode::EmptyPassword;
        return ticket;
    }

    if secret.is_empty() && secret.len() > 5 && (password.is_empty() || password.len() < 64) {
        ticket.result = ResultCode::InvalidPassword;
        return ticket;
    }

    if !secret.is_empty() && secret != "?" && secret.len() < 6 {
        ticket.result = ResultCode::InvalidSecret;
        return ticket;
    }

    lazy_static! {
        static ref RE: Regex = Regex::new("[-]").unwrap();
    }

    let query = format!("'v-s:login' == '{}'", RE.replace_all(login, " +"));

    let candidate_account_ids = module.fts.query(FTQuery::new_with_ticket(systicket, &query));
    if candidate_account_ids.result_code == 200 && candidate_account_ids.count > 0 {
        for account_id in &candidate_account_ids.result {
            if let Some(account) = module.get_individual(&account_id, &mut Individual::default()) {
                let user_id = account.get_first_literal("v-s:owner").unwrap_or_default();
                if user_id.is_empty() {
                    error!("user id is null, user_indv={}", account);
                    continue;
                }

                let user_login = account.get_first_literal("v-s:login").unwrap_or_default();
                if user_login.is_empty() {
                    error!("user login {:?} not equal request login {}", user_login, login);
                    continue;
                }

                if user_login.to_lowercase() != login.to_lowercase() {
                    error!("user login {} not equal request login {}", user_login, login);
                    continue;
                }

                let mut person = Individual::default();
                if module.get_individual(&user_id, &mut person).is_none() {
                    error!("user {} not found", user_id);
                    continue;
                }

                let now = Utc::now().naive_utc().timestamp();
                let mut exist_password = String::default();
                let mut edited = now;
                let mut uses_credential = Individual::default();

                match account.get_first_literal("v-s:usesCredential") {
                    Some(uses_credential_uri) => {
                        if let Some(uses_credential) = module.get_individual(&user_id, &mut uses_credential) {
                            exist_password = uses_credential.get_first_literal("v-s:password").unwrap_or_default();
                            edited = uses_credential.get_first_datetime("v-s:dateFrom").unwrap_or_default();
                        }
                    }
                    None => {
                        exist_password = account.get_first_literal("v-s:password").unwrap_or_default();
                        edited = now;

                        uses_credential.set_id(&(account_id.to_owned() + "-crdt"));
                        uses_credential.set_uri("rdf:type", "v-s:Credential");
                        uses_credential.set_string("v-s:password", &exist_password, Lang::NONE);
                        uses_credential.set_datetime("v-s:dateFrom", edited);

                        let res = module.api.update(&systicket, IndvOp::Put, &mut uses_credential);
                        if res.result != ResultCode::Ok {
                            error!("fail update, uri={}, result_code={:?}", uses_credential.get_id(), res.result);
                            continue;
                        } else {
                            info!("create v-s:Credential{}, res={:?}", uses_credential.get_id(), res);

                            account.remove("v-s:password");
                            account.set_uri("v-s:usesCredential", uses_credential.get_id());

                            let res = module.api.update(&systicket, IndvOp::Put, account);
                            if res.result != ResultCode::Ok {
                                error!("fail update, uri={}, res={:?}", account.get_id(), res);
                                continue;
                            }
                            info!("update user {}, res={:?}", account.get_id(), res);
                        }
                    }
                }

                let origin = person.get_first_literal("v-s:origin");
                let old_secret = uses_credential.get_first_literal("v-s:secret").unwrap_or_default();

                if !secret.is_empty() && secret.len() > 5 {
                    if old_secret.is_empty() {
                        error!("update password: secret not found, user={}", person.get_id());
                        ticket.result = ResultCode::InvalidSecret;
                        remove_secret(&mut uses_credential, person.get_id(), module, systicket);
                        return ticket;
                    }

                    if secret != old_secret {
                        error!("request for update password: send secret not equal request secret {}, user={}", secret, person.get_id());
                        ticket.result = ResultCode::InvalidSecret;
                        remove_secret(&mut uses_credential, person.get_id(), module, systicket);
                        return ticket;
                    }

                    let prev_secret_date = uses_credential.get_first_datetime("v-s:SecretDateFrom").unwrap_or_default();
                    if now - prev_secret_date > 12 * 60 * 60 {
                        ticket.result = ResultCode::SecretExpired;
                        error!("request new password, secret expired, login={} password={} secret={}", login, password, secret);
                        return ticket;
                    }

                    if exist_password == password {
                        error!("update password: now password equal previous password, reject. user={}", person.get_id());
                        ticket.result = ResultCode::NewPasswordIsEqualToOld;
                        remove_secret(&mut uses_credential, person.get_id(), module, systicket);
                        return ticket;
                    }

                    if password == EMPTY_SHA256_HASH {
                        error!("update password: now password is empty, reject. user={}", person.get_id());
                        ticket.result = ResultCode::EmptyPassword;
                        remove_secret(&mut uses_credential, person.get_id(), module, systicket);
                        return ticket;
                    }

                    // update password
                    uses_credential.set_string("v-s:password", &password, Lang::NONE);
                    uses_credential.set_datetime("v-s:dateFrom", edited);
                    uses_credential.remove("v-s:secret");

                    let res = module.api.update(&systicket, IndvOp::Put, &mut uses_credential);
                    if res.result != ResultCode::Ok {
                        ticket.result = ResultCode::AuthenticationFailed;
                        error!("fail store new password {} for user, user={}", password, person.get_id());
                    } else {
                        //ticket = create_new_ticket(login, user_id);
                        info!("update password {} for user, user={}", password, person.get_id());
                    }
                    return ticket;
                } else {
                    let mut is_request_new_password = false;
                    if pass_lifetime > 0 {
                        if now - edited > pass_lifetime {
                            error!("password is old, lifetime > {} days, user={}", pass_lifetime / 60 / 60 / 24, account.get_id());
                            is_request_new_password = true;
                        }
                    }

                    if secret == "?" {
                        error!("request for new password, user={}", account.get_id());
                        is_request_new_password = true;
                    }

                    if (is_request_new_password == true) {
                        error!("request new password, login={} password={} secret={}", login, password, secret);
                        ticket.result = ResultCode::PasswordExpired;

                        let n_secret = thread_rng().gen_range(100000, 999999).to_string();
                        if !n_secret.is_empty() {
                            let prev_secret_date = uses_credential.get_first_datetime("v-s:SecretDateFrom").unwrap_or_default();
                            if now - prev_secret_date < 10 * 60 {
                                ticket.result = ResultCode::TooManyRequests;
                                error!("request new password, to many request, login={} password={} secret={}", login, password, secret);
                                return ticket;
                            }
                        }

                        uses_credential.set_string("v-s:secret", &n_secret, Lang::NONE);
                        uses_credential.set_datetime("v-s:SecretDateFrom", now);

                        let res = module.api.update(&systicket, IndvOp::Put, &mut uses_credential);
                        if res.result != ResultCode::Ok {
                            ticket.result = ResultCode::AuthenticationFailed;
                            error!("fail store new secret, user={}", person.get_id());
                            return ticket;
                        }

                        let mailbox = account.get_first_literal("v-s:mailbox").unwrap_or_default();

                        if !mailbox.is_empty() && mailbox.len() > 3 {
                            let mut mail_with_secret = Individual::default();

                            let uuid1 = "d:mail_".to_owned() + &Uuid::new_v4().to_hyphenated().to_string();
                            mail_with_secret.set_id(&uuid1);
                            mail_with_secret.add_uri("rdf:type", "v-s:Email");
                            mail_with_secret.add_string("v-s:recipientMailbox", &mailbox, Lang::NONE);
                            mail_with_secret.add_datetime("v-s:created", now);
                            mail_with_secret.add_string("v-s:messageBody", &("your secret code is ".to_owned() + &n_secret), Lang::NONE);

                            let res = module.api.update(&systicket, IndvOp::Put, &mut mail_with_secret);
                            if res.result != ResultCode::Ok {
                                ticket.result = ResultCode::AuthenticationFailed;
                                error!("fail store email with new secret, user={}", account.get_id());
                                return ticket;
                            } else {
                                info!("send {} new secret {} to mailbox {}, user={}", mail_with_secret.get_id(), n_secret, mailbox, account.get_id())
                            }
                        } else {
                            error!("mailbox not found, user={}", account.get_id());
                        }
                        return ticket;
                    }

                    if !exist_password.is_empty() && !password.is_empty() && password.len() > 63 && exist_password == password {
                        //ticket = create_new_ticket(login, user_id);
                        return ticket;
                    } else {
                        warn!("request passw not equal with exist, user={}", account.get_id());
                    }
                }
                warn!("user {} not pass", account.get_id());
            } else {
                error!("fail read, uri={}", &account_id);
            }
        }
    }

    error!("fail authenticate, login={} password={}, candidate users={:?}", login, password, candidate_account_ids.result);
    ticket.result = ResultCode::AuthenticationFailed;
    ticket
}

fn get_password_lifetime(module: &mut Module) -> Option<i64> {
    let mut node = Individual::default();
    if module.storage.get_individual("cfg:standart_node", &mut node) {
        return node.get_first_integer("cfg:user_password_lifetime");
    }
    None
}

fn remove_secret(uses_credential: &mut Individual, person_id: &str, module: &mut Module, sticket: &str) {}
