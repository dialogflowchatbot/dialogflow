use super::context::Context;
use super::dto::{Request, Response};
use crate::ai::completion::Prompt;
use crate::flow::rt::dto::UserInputResult;
use crate::flow::rt::node::RuntimeNode;
use crate::intent::detector;
use crate::result::{Error, Result};

pub(in crate::flow::rt) async fn process(req: &mut Request) -> Result<Response> {
    // log::info!("user input: {}", &req.user_input);
    // let now = std::time::Instant::now();
    if req.session_id.is_empty() {
        req.session_id = scru128::new_string();
    }
    let mut ctx = Context::get(&req.robot_id, &req.session_id);
    // log::info!("get ctx {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    if ctx.no_node() {
        if ctx.main_flow_id.is_empty() {
            ctx.main_flow_id.push_str(&req.main_flow_id);
        }
        ctx.add_node(&req.main_flow_id);
    }
    // log::info!("add_node time {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    if req.user_input_intent.is_none()
        && req.user_input_result == UserInputResult::Successful
        && !req.user_input.is_empty()
    {
        req.user_input_intent = detector::detect(&req.robot_id, &req.user_input).await?;
        // println!("{:?}", req.user_input_intent);
    }
    // log::info!("Intent detection took {:?}", now.elapsed());
    if !req.import_variables.is_empty() {
        for v in req.import_variables.iter_mut() {
            let k = std::mem::take(&mut v.var_name);
            let v = crate::variable::dto::VariableValue::new(&v.var_val, &v.var_type);
            ctx.vars.insert(k, v);
        }
    }
    // println!("intent detect {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    ctx.chat_history.push(Prompt {
        role: String::from("user"),
        content: req.user_input.clone(),
    });
    let r = exec(req, &mut ctx);
    if r.is_ok() {
        let res = r.as_ref().unwrap();
        if !res.answers.is_empty() {
            for a in res.answers.iter() {
                ctx.chat_history.push(Prompt {
                    role: String::from("assistant"),
                    content: a.content.clone(),
                });
            }
        }
    }
    // println!("exec {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    ctx.save()?;
    // log::info!("ctx save time {:?}", now.elapsed());
    r
}

pub(in crate::flow::rt) fn exec(req: &Request, ctx: &mut Context) -> Result<Response> {
    // let now = std::time::Instant::now();
    let mut response = Response::new(req);
    for _i in 0..100 {
        // let now = std::time::Instant::now();
        if let Some(mut n) = ctx.pop_node() {
            // println!("pop node {:?}", now.elapsed());
            let ret = n.exec(&req, ctx, &mut response);
            // println!("node exec {:?}", now.elapsed());
            if ret {
                // log::info!("exec time {:?}", now.elapsed());
                return Ok(response);
            }
        } else {
            return Ok(response);
        }
    }
    Err(Error::ErrorWithMessage(String::from(
        "执行次数太多，请检查流程配置是否正确。",
    )))
}
