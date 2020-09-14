use crate::process_source::IndexedNodeTree;
use crate::Context;
use indextree::NodeId;
use rusty_v8::{ContextScope, Integer};
use v_onto::individual::Individual;
use v_v8::callback::*;
use v_v8::common::v8obj_into_individual;
use v_v8::scripts_workplace::{ScriptInfo, ScriptsWorkPlace};
use v_v8::session_cache::CallbackSharedData;

pub(crate) struct ScriptInfoContext {}

impl Default for ScriptInfoContext {
    fn default() -> Self {
        Self {}
    }
}

pub enum OutValue {
    Bool(bool),
    List(Vec<String>),
    Individual(Individual),
    None,
}

pub fn execute_js(
    process: &mut Individual,
    token: &mut Individual,
    script_id: &str,
    tag_with_js: &str,
    js_idx: &NodeId,
    work_order_uri: Option<&str>,
    nt: &IndexedNodeTree,
    ctx: &mut Context,
    out: &mut OutValue,
) -> bool {
    let compiled_script = if let Some(script) = ctx.workplace.scripts.get(script_id) {
        script.compiled_script
    } else {
        if let Some(script_text) = nt.get_values_of_tag(js_idx, tag_with_js).get(0) {
            if let OutValue::None = out {
                prepare_script(&mut ctx.workplace, &script_id, script_text);
            } else {
                prepare_eval_script(&mut ctx.workplace, &script_id, script_text);
            }

            if let Some(s) = ctx.workplace.scripts.get(script_id) {
                s.compiled_script
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(c) = compiled_script {
        let mut session_data = CallbackSharedData::default();
        session_data.g_key2attr.insert("$ticket".to_owned(), ctx.sys_ticket.to_owned());
        session_data.g_key2indv.insert("$process".to_owned(), Individual::new_from_obj(process.get_obj()));
        session_data.g_key2indv.insert("$token".to_owned(), Individual::new_from_obj(token.get_obj()));
        if let Some(w) = work_order_uri {
            session_data.g_key2attr.insert("$work_order".to_owned(), w.to_owned());
        }

        let mut sh_g_vars = G_VARS.lock().unwrap();
        let g_vars = sh_g_vars.get_mut();
        *g_vars = session_data;
        drop(sh_g_vars);

        let hs = ContextScope::new(&mut ctx.workplace.scope, ctx.workplace.context);
        let mut local_scope = hs;

        if let Some(res) = c.run(&mut local_scope) {
            match out {
                OutValue::Bool(ov) => {
                    if res.is_boolean() {
                        if res.to_integer(local_scope.as_mut()).unwrap().value() != 0 {
                            *ov = true;
                        } else {
                            *ov = false;
                        }
                        return true;
                    }
                }
                OutValue::List(ov) => {
                    if let Some(obj) = res.to_object(&mut local_scope) {
                        if let Some(key_list) = obj.get_property_names(&mut local_scope) {
                            for resources_idx in 0..key_list.length() {
                                let j_resources_idx = Integer::new(&mut local_scope, resources_idx as i32);
                                if let Some(v) = obj.get(&mut local_scope, j_resources_idx.into()) {
                                    if let Some(s) = v.to_string(&mut local_scope) {
                                        let ss = s.to_rust_string_lossy(&mut local_scope);
                                        ov.push(ss);
                                    }
                                }
                            }
                            return true;
                        }
                    }
                }
                OutValue::Individual(v) => {
                    if let Some(obj) = res.to_object(&mut local_scope) {
                        v8obj_into_individual(&mut local_scope, obj, v);
                        return true;
                    }
                }
                _ => {}
            }
        }
    }
    false
}

pub(crate) fn prepare_script(wp: &mut ScriptsWorkPlace<ScriptInfoContext>, script_id: &str, script_text: &str) {
    let str_script = "try { var ticket = get_env_str_var ('$ticket'); script(); function script() {".to_owned() + script_text + "}; } catch (e) { log_trace (e); }";

    let mut scr_inf: ScriptInfo<ScriptInfoContext> = ScriptInfo::new_with_src(script_id, &str_script);

    wp.add_to_order(&scr_inf);

    let scope = &mut ContextScope::new(&mut wp.scope, wp.context);
    scr_inf.compile_script(scope);
    wp.scripts.insert(scr_inf.id.to_string(), scr_inf);
}

pub(crate) fn prepare_eval_script(wp: &mut ScriptsWorkPlace<ScriptInfoContext>, script_id: &str, script_text: &str) {
    let str_script = &script_text;

    let mut scr_inf: ScriptInfo<ScriptInfoContext> = ScriptInfo::new_with_src(script_id, &str_script);

    wp.add_to_order(&scr_inf);

    let scope = &mut ContextScope::new(&mut wp.scope, wp.context);
    scr_inf.compile_script(scope);
    wp.scripts.insert(scr_inf.id.to_string(), scr_inf);
}