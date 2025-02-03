use std::collections::{HashMap, LinkedList};
// use std::rc::Rc;
// use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

// use erased_serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};

use super::node::RuntimeNnodeEnum;
use crate::ai::completion::Prompt;
use crate::db;
use crate::man::settings;
use crate::result::Result;
use crate::variable::dto::VariableValue;

const TABLE: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new("contexts");
pub(crate) const CONTEXT_KEY: &str = "contexts";
// const LOCKER: OnceLock<Mutex<()>> = OnceLock::new();

// #[derive(Deserialize, Serialize)]
// pub(crate) struct ContextStatus {
//     session_id: String,
// }

#[derive(Deserialize, Serialize)]
pub(crate) struct Context {
    robot_id: String,
    pub(in crate::flow::rt) main_flow_id: String,
    session_id: String,
    pub(in crate::flow::rt) node: Option<Vec<u8>>,
    pub(in crate::flow::rt) nodes: LinkedList<String>,
    pub(crate) vars: HashMap<String, VariableValue>,
    #[serde(skip)]
    pub(crate) none_persistent_vars: HashMap<String, VariableValue>,
    #[serde(skip)]
    pub(crate) none_persistent_data: HashMap<String, String>,
    last_active_time: u64,
    pub(crate) chat_history: Vec<Prompt>,
}

impl Context {
    pub(crate) fn get(robot_id: &str, session_id: &str) -> Self {
        let r: Result<Option<Context>> = db::query(TABLE, session_id);
        if let Ok(o) = r {
            if let Some(mut ctx) = o {
                ctx.last_active_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                return ctx;
            }
        }
        let r: Result<Option<Vec<String>>> = db::query(TABLE, CONTEXT_KEY);
        if let Ok(op) = r {
            if let Some(mut d) = op {
                d.push(String::from(session_id));
                if let Err(e) = db::write(TABLE, CONTEXT_KEY, &d) {
                    eprint!("{:?}", e);
                }
            }
        }
        let ctx = Self {
            robot_id: String::from(robot_id),
            main_flow_id: String::with_capacity(64),
            session_id: String::from(session_id),
            node: None,
            nodes: LinkedList::new(),
            vars: HashMap::with_capacity(16),
            none_persistent_vars: HashMap::with_capacity(16),
            none_persistent_data: HashMap::with_capacity(16),
            last_active_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            chat_history: Vec::with_capacity(16),
        };
        ctx
    }

    pub(crate) fn save(&self) -> Result<()> {
        db::write(TABLE, self.session_id.as_str(), self)
    }

    // pub(crate) fn clear(&mut self) -> Result<()> {
    //     self.nodes.clear();
    //     self.vars.clear();
    //     self.save()
    // }

    pub(in crate::flow::rt) fn no_node(&self) -> bool {
        self.node.is_none() && self.nodes.is_empty()
    }

    pub(in crate::flow::rt) fn add_node(&mut self, node_id: &str) {
        // print!("add_node {} ", node_id);
        self.nodes.push_front(String::from(node_id));
        // let now = std::time::Instant::now();
        // if let Ok(r) = db::get_runtime_node(node_id) {
        //     if let Some(n) = r {
        //         self.nodes.push_front(n);
        //         // println!("added");
        //     }
        // }
        // println!("add_node used time:{:?}", now.elapsed());
    }

    pub(in crate::flow::rt) fn pop_node(&mut self) -> Option<RuntimeNnodeEnum> {
        // log::info!("nodes len {}", self.nodes.len());
        // let now = std::time::Instant::now();
        if self.node.is_some() {
            let node = std::mem::replace(&mut self.node, None);
            let v = node.unwrap();
            match crate::flow::rt::node::deser_node(v.as_ref()) {
                Ok(n) => return Some(n),
                Err(e) => {
                    log::error!("pop_node failed err: {:?}", &e);
                }
            }
        }
        if let Some(node_id) = self.nodes.pop_front() {
            // log::info!("main_flow_id {} node_id {}", &self.main_flow_id, &node_id);
            if let Ok(r) = super::crud::get_runtime_node(&self.main_flow_id, &node_id) {
                // log::info!("pop_node time {:?}", now.elapsed());
                return r;
            }
        }
        None
    }
}

pub(crate) fn init() -> Result<()> {
    let d: Vec<String> = vec![];
    db::write(TABLE, CONTEXT_KEY, &d)
}

fn session_not_expired(d: &mut Vec<String>, idx: usize, now: u64) -> Result<bool> {
    let session_id = d[idx].as_str();
    // let session_id_ref:&str =session_id.as_ref();
    let ctx: Option<Context> = db::query(TABLE, session_id)?;
    if ctx.is_none() {
        d.remove(idx);
        return Ok(false);
    }
    let c = ctx.as_ref().unwrap();
    let settings = settings::get_settings(&c.robot_id)?;
    if settings.is_none() {
        if let Err(e) = db::remove(TABLE, session_id) {
            log::warn!("Discarding expired session {} failed {:?}", session_id, e);
        } else {
            d.remove(idx);
        }
        return Ok(false);
    }
    if now - c.last_active_time
        > 86400u64 /* 1 day */
            .min(settings.as_ref().unwrap().max_session_idle_sec as u64)
    {
        if let Err(e) = db::remove(TABLE, session_id) {
            log::warn!("Discarding expired session {} failed {:?}", session_id, e);
        } else {
            log::info!("Discarded expired session: {}", session_id);
            d.remove(idx);
        }
        return Ok(false);
    }
    Ok(true)
}

pub async fn clean_expired_session(mut recv: tokio::sync::oneshot::Receiver<()>) {
    let mut interval = interval(Duration::from_secs(60));
    loop {
        // https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
        // https://users.rust-lang.org/t/how-can-i-terminate-a-tokio-task-even-if-its-not-finished/40641
        tokio::select! {
          _ = interval.tick() => {
          }
          _ = &mut recv => {
            break;
          }
        }
        // sleep(Duration::from_millis(1800000)).await;
        // log::info!("Cleaning expired sessions");
        let r: Option<Vec<String>> = db::query(TABLE, CONTEXT_KEY)
            .expect("Please remove ./data/flow.db file and restart this application.");
        if let Some(mut d) = r {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(dura) => {
                    let now = dura.as_secs();
                    let mut i = 0;
                    while i < d.len() {
                        // println!("{} {}", now, d[i].create_time);
                        let session_not_expired = session_not_expired(&mut d, i, now);
                        if session_not_expired.is_ok() && session_not_expired.unwrap() {
                            i += 1;
                        }
                    }
                    if let Err(e) = db::write(TABLE, CONTEXT_KEY, &d) {
                        log::error!("{:?}", e);
                    }
                }
                Err(e) => log::error!("{:?}", e),
            }
        }
    }
}
