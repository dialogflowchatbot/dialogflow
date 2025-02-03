use rkyv::{Archive, Deserialize, Serialize};

use super::condition::ConditionData;
use super::context::Context;
use super::dto::{CollectData, Request, Response};
use crate::flow::canvas::dto::NextActionType;
use crate::flow::rt::collector;
use crate::result::{Error, Result};
use crate::variable::crud as variable;
use crate::variable::dto::{VariableType, VariableValue};

const VAR_WRAP_SYMBOL: char = '`';

#[repr(u8)]
#[derive(PartialEq)]
pub(in crate::flow::rt) enum RuntimeNodeTypeId {
    TextNode = 1,
    GotoAnotherNode = 2,
    CollectNode = 3,
    ConditionNode = 4,
    TerminateNode = 5,
}

pub(crate) type BoxedRuntimeNode = Box<dyn RuntimeNode>;

pub(crate) trait RuntimeNode {
    fn exec(&self, req: &Request, ctx: &mut Context, response: &mut Response) -> bool;
}

fn replace_vars(text: &str, req: &Request, ctx: &Context) -> Result<String> {
    let mut new_str = String::with_capacity(128);
    let mut start = 0usize;
    loop {
        if let Some(mut begin) = text[start..].find(VAR_WRAP_SYMBOL) {
            begin = start + begin;
            new_str.push_str(&text[start..begin]);
            if let Some(mut end) = text[begin + 1..].find(VAR_WRAP_SYMBOL) {
                end = begin + end + 1;
                // println!("{} {} {} {}", &text[begin + 1..],start, begin,end);
                let var = variable::get(&text[begin + 1..end])?;
                if let Some(v) = var {
                    let value = v.get_value(req, ctx);
                    new_str.push_str(&value);
                    start = end + 1;
                } else {
                    new_str.push_str(&text[begin..end]);
                    start = end;
                }
                // new_str.push_str(&variable::get_value(&text[begin + 1..end - 1], req, ctx));
            } else {
                start = begin;
                break;
            }
        } else {
            break;
        }
    }
    new_str.push_str(&text[start..]);
    Ok(new_str)
}

#[inline]
fn add_next_node(ctx: &mut Context, next_node_id: &str) {
    ctx.add_node(next_node_id);
}

#[derive(Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq), check_bytes)]
pub(in crate::flow::rt) struct TextNode {
    pub(super) text: String,
    pub(super) ret: bool,
    pub(super) next_node_id: String,
}

impl RuntimeNode for TextNode {
    fn exec(&self, req: &Request, ctx: &mut Context, response: &mut Response) -> bool {
        // println!("Into TextNode");
        // let now = std::time::Instant::now();
        match replace_vars(&self.text, &req, &ctx) {
            Ok(answer) => response.answers.push(answer),
            Err(e) => log::error!("{:?}", e),
        };
        add_next_node(ctx, &self.next_node_id);
        // println!("TextNode used time:{:?}", now.elapsed());
        self.ret
    }
}

#[derive(Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq), check_bytes)]
pub(in crate::flow::rt) struct GotoAnotherNode {
    pub(super) next_node_id: String,
}

impl RuntimeNode for GotoAnotherNode {
    fn exec(&self, _req: &Request, ctx: &mut Context, _response: &mut Response) -> bool {
        // println!("Into GotoAnotherNode");
        add_next_node(ctx, &self.next_node_id);
        false
    }
}

#[derive(Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq), check_bytes)]
pub(in crate::flow::rt) struct CollectNode {
    pub(super) var_name: String,
    pub(super) collect_type: collector::CollectType,
    pub(super) successful_node_id: String,
    pub(super) failed_node_id: String,
}

impl RuntimeNode for CollectNode {
    fn exec(&self, req: &Request, ctx: &mut Context, response: &mut Response) -> bool {
        // println!("Into CollectNode");
        if let Some(r) = collector::collect(&req.user_input, &self.collect_type) {
            let v = VariableValue {
                var_name: self.var_name.clone(),
                var_type: VariableType::String,
                var_value: String::from(r),
            };
            ctx.vars.insert(self.var_name.clone(), v);
            let collect_data = CollectData {
                var_name: self.var_name.clone(),
                value: String::from(r),
            };
            response.collect_data.push(collect_data);
            add_next_node(ctx, &self.successful_node_id);
            // println!("{} {}", r, &self.successful_node_id);
        } else {
            add_next_node(ctx, &self.failed_node_id);
        }
        false
    }
}

#[derive(Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq), check_bytes)]
pub(in crate::flow::rt) struct ConditionNode {
    pub(super) next_node_id: String,
    pub(super) goto_node_id: String,
    pub(super) conditions: Vec<Vec<ConditionData>>,
}

impl RuntimeNode for ConditionNode {
    fn exec(&self, req: &Request, ctx: &mut Context, _response: &mut Response) -> bool {
        // println!("Into ConditionNode");
        let mut r = false;
        for and_conditions in self.conditions.iter() {
            for cond in and_conditions.iter() {
                r = cond.compare(req, ctx);
                if !r {
                    break;
                }
            }
            if r {
                add_next_node(ctx, &self.goto_node_id);
                return false;
            }
        }
        add_next_node(ctx, &self.next_node_id);
        false
    }
}

#[derive(Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq), check_bytes)]
pub(in crate::flow::rt) struct TerminateNode {}

impl RuntimeNode for TerminateNode {
    fn exec(&self, _req: &Request, _ctx: &mut Context, response: &mut Response) -> bool {
        // println!("Into TerminateNode");
        response.next_action = NextActionType::Terminate;
        true
    }
}

pub(crate) fn deser_node(bytes: &[u8]) -> Result<Box<dyn RuntimeNode>> {
    if let Some(&node_type) = bytes.last() {
        if node_type == RuntimeNodeTypeId::TextNode as u8 {
            let mut vec = rkyv::AlignedVec::new();
            vec.extend_from_slice(&bytes[..bytes.len() - 1]);
            let archived = rkyv::check_archived_root::<TextNode>(&vec).unwrap();
            let deserialized: TextNode = archived.deserialize(&mut rkyv::Infallible).unwrap();
            return Ok(Box::new(deserialized));
        }
        if node_type == RuntimeNodeTypeId::ConditionNode as u8 {
            let mut vec = rkyv::AlignedVec::new();
            vec.extend_from_slice(&bytes[..bytes.len() - 1]);
            let archived = rkyv::check_archived_root::<ConditionNode>(&vec).unwrap();
            let deserialized: ConditionNode = archived.deserialize(&mut rkyv::Infallible).unwrap();
            return Ok(Box::new(deserialized));
        }
        if node_type == RuntimeNodeTypeId::CollectNode as u8 {
            let mut vec = rkyv::AlignedVec::new();
            vec.extend_from_slice(&bytes[..bytes.len() - 1]);
            let archived = rkyv::check_archived_root::<CollectNode>(&vec).unwrap();
            let deserialized: CollectNode = archived.deserialize(&mut rkyv::Infallible).unwrap();
            return Ok(Box::new(deserialized));
        }
        if node_type == RuntimeNodeTypeId::GotoAnotherNode as u8 {
            let mut vec = rkyv::AlignedVec::new();
            vec.extend_from_slice(&bytes[..bytes.len() - 1]);
            let archived = rkyv::check_archived_root::<GotoAnotherNode>(&vec).unwrap();
            let deserialized: GotoAnotherNode =
                archived.deserialize(&mut rkyv::Infallible).unwrap();
            return Ok(Box::new(deserialized));
        }
        if node_type == RuntimeNodeTypeId::TerminateNode as u8 {
            let mut vec = rkyv::AlignedVec::new();
            vec.extend_from_slice(&bytes[..bytes.len() - 1]);
            let archived = rkyv::check_archived_root::<TerminateNode>(&vec).unwrap();
            let deserialized: TerminateNode = archived.deserialize(&mut rkyv::Infallible).unwrap();
            return Ok(Box::new(deserialized));
        }
        return Err(Error::ErrorWithMessage(String::from("未知的执行节点数据")));
    }
    Err(Error::ErrorWithMessage(String::from("执行节点数据为空")))
}
