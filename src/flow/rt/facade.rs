use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use axum::Json;
use axum::response::IntoResponse;
use tokio::sync::mpsc::Sender;

use super::dto::Request;
use super::executor;
use crate::result::Result;
use crate::web::server::to_res;

static ANSWER_SSE_SESSIONS: LazyLock<Mutex<HashMap<String, Sender<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::with_capacity(128)));

pub(crate) async fn answer(Json(mut req): Json<Request>) -> impl IntoResponse {
    let now = std::time::Instant::now();
    let r = executor::process(&mut req).await;
    // println!("exec used time:{:?}", now.elapsed());
    let res = to_res(r);
    log::info!("Response used time:{:?}", now.elapsed());
    res
}

pub(crate) async fn answer_sse(Json(req): Json<Request>) -> impl IntoResponse {
    let now = std::time::Instant::now();
    let (s, r) = tokio::sync::mpsc::channel::<String>(1);
    let mut l = ANSWER_SSE_SESSIONS.lock().unwrap();
    l.insert(String::new(), s);
    log::info!("Response used time:{:?}", now.elapsed());
    ""
}

pub(super) fn get_sender(session_id: &str) -> Result<Option<Sender<String>>> {
    let l = ANSWER_SSE_SESSIONS.lock()?;
    if l.contains_key(session_id) {
        let s = l.get(session_id).unwrap();
        return Ok(Some(s.clone()));
    }
    return Ok(None);
}
