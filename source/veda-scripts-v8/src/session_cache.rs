use std::collections::{HashMap, HashSet};
use std::string::ToString;
use v_api::app::ResultCode;
use v_api::{IndvOp, APIClient};
use v_module::module::indv_apply_cmd;
use v_onto::individual::Individual;
use v_onto::onto::Onto;
use v_onto::parser::parse_raw;
use v_storage::inproc_indv_r_storage::get_individual;

pub(crate) struct CallbackSharedData {
    pub g_super_classes: String,
    pub g_parent_script_id: String,
    pub g_parent_document_id: String,
    pub g_prev_state: Option<Individual>,
    pub g_user: String,
    pub g_event_id: String,
    //_Buff   g_execute_script;
    pub g_document: Option<Individual>,
    pub g_uri: String,
    pub g_ticket: String,
}

impl Default for CallbackSharedData {
    fn default() -> Self {
        Self {
            g_super_classes: "".to_string(),
            g_parent_script_id: "".to_string(),
            g_parent_document_id: "".to_string(),
            g_prev_state: None,
            g_user: "".to_string(),
            g_event_id: "".to_string(),
            g_document: None,
            g_uri: "".to_string(),
            g_ticket: "".to_string(),
        }
    }
}

impl CallbackSharedData {
    pub(crate) fn set_g_parent_script_id_etc(&mut self, event_id: &str) {
        let mut event_id = event_id;

        if !event_id.is_empty() {
            let mut aa: Vec<&str> = event_id.split(";").collect();
            if aa.len() > 0 {
                event_id = aa.get(0).unwrap();
            }

            aa = event_id.split("+").collect();

            if aa.len() >= 2 {
                self.g_parent_script_id = aa.get(1).unwrap().to_string();
                self.g_parent_document_id = aa.get(0).unwrap().to_string();
            } else {
                self.g_parent_script_id = String::default();
                self.g_parent_document_id = String::default();
            }
        } else {
            self.g_parent_script_id = String::default();
            self.g_parent_document_id = String::default();
        }
    }

    pub(crate) fn set_g_super_classes(&mut self, indv_types: &Vec<String>, onto: &Onto) {
        let mut super_classes = HashSet::new();

        for indv_type in indv_types.iter() {
            onto.get_subs(indv_type, &mut super_classes);
        }

        self.g_super_classes = String::new();

        for s in super_classes.iter() {
            if !self.g_super_classes.is_empty() {
                self.g_super_classes.push(',');
            }
            self.g_super_classes.push_str(s);
        }
    }
}

pub(crate) struct TransactionItem {
    op_id: u64,
    uri: String,
    cmd: IndvOp,
    indv: Individual,
    ticket_id: String,
    event_id: String,
    user_uri: String,
    rc: ResultCode,
}

pub(crate) struct Transaction {
    buff: HashMap<String, usize>,
    queue: Vec<TransactionItem>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            buff: Default::default(),
            queue: vec![],
        }
    }
}

impl Transaction {
    fn add_item(&mut self, item: TransactionItem) {
        self.buff.insert(item.uri.clone(), self.queue.len());
        self.queue.push(item);
    }

    fn get_indv(&mut self, id: &str) -> Option<&mut Individual> {
        if let Some(idx) = self.buff.get(id) {
            if let Some(ti) = self.queue.get_mut(*idx) {
                return Some(&mut ti.indv);
            }
        }

        None
    }

    pub(crate) fn add_to_transaction(&mut self, cmd: IndvOp, new_indv: Individual, ticket_id: String, user_id: String) -> ResultCode {
        let mut ti = TransactionItem {
            op_id: 0,
            uri: "".to_string(),
            cmd,
            indv: new_indv,
            ticket_id,
            event_id: "".to_string(),
            user_uri: user_id,
            rc: ResultCode::Ok,
        };

        if ti.cmd == IndvOp::Remove {
        } else {
            ti.uri = ti.indv.get_id().to_string();

            if ti.cmd == IndvOp::AddIn || ti.cmd == IndvOp::SetIn || ti.cmd == IndvOp::RemoveFrom {
                if let Some(mut prev_indv) = self.get_indv(ti.indv.get_id()) {
                    info!("BEFORE: prev_indv={}", &prev_indv);
                    indv_apply_cmd(&ti.cmd, &mut prev_indv, &mut ti.indv);
                    info!("AFTER: prev_indv={}", &prev_indv);
                } else {
                    if let Some(mut prev_indv) = get_individual(ti.indv.get_id()) {
                        if parse_raw(&mut prev_indv).is_ok() {
                            prev_indv.parse_all();
                            info!("BEFORE: prev_indv={}", &prev_indv);
                            indv_apply_cmd(&ti.cmd, &mut prev_indv, &mut ti.indv);
                            info!("AFTER: prev_indv={}", &prev_indv);
                            ti.indv = prev_indv;
                        } else {
                            ti.rc = ResultCode::UnprocessableEntity;
                        }
                    } else {
                        ti.rc = ResultCode::UnprocessableEntity;
                    }
                }
                ti.cmd = IndvOp::Put;
            }
        }

        if ti.rc == ResultCode::Ok {
            self.add_item(ti);
            ResultCode::Ok
        } else {
            ti.rc
        }
    }

}

pub(crate) fn commit (tnx: &Transaction, api_client: &mut APIClient) -> ResultCode{

    for ti in tnx.queue.iter() {
        if ti.cmd == IndvOp::Remove && ti.indv.is_empty() {
            continue;
        }

        if ti.rc != ResultCode::Ok {
            return ti.rc;
        }

        let res = api_client.update(&ti.ticket_id, ti.cmd.clone(), &ti.indv);
    }

    ResultCode::Ok
}