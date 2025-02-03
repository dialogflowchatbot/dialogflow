use core::time::Duration;
use std::convert::Infallible;
use std::result::Result;

use axum::body::Bytes;
use axum::response::sse::{Event, KeepAlive, Sse};
// use crossbeam_channel::bounded;
use futures::future::Either;
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt as _;

use crate::ai::completion;

#[derive(Deserialize, Serialize)]
pub(crate) struct Request {
    pub(crate) robot_id: String,
    pub(crate) prompt: String,
}

// struct Guard;
// impl Drop for Guard {
//     fn drop(&mut self) {
//         println!("A SSE connection was dropped!")
//     }
// }

pub(crate) async fn gen_text(bytes: Bytes) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let q: Request = serde_json::from_slice(bytes.as_ref()).unwrap();
    // let _guard = Guard;
    let stream = if q.robot_id.is_empty() || q.prompt.is_empty() {
        Either::Left(stream::once(futures::future::ready(
            Ok::<Event, Infallible>(Event::default().data("Invalid robot_id or prompt")),
        )))
    } else {
        // let (sender, receiver) = bounded::<String>(1);
        // Either::Right(stream::once(futures::future::ready(Ok::<Event, Infallible>(
        //     Event::default().data("Invalid robot_id or prompt")
        // ))))
        let (sender, receiver) = mpsc::channel::<String>(5);
        let stream = ReceiverStream::new(receiver).map(|s| {
            // log::info!("Sse sending {s}");
            let event = Event::default().data(s);
            Ok::<Event, Infallible>(event)
        });
        tokio::spawn(async move {
            let borrowed_sender = &sender;
            if let Err(e) = completion::completion(&q.robot_id, &q.prompt, borrowed_sender).await {
                log::error!("{:?}", &e);
            }
        });
        Either::Right(stream)
    };
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive-text"),
    )
}
