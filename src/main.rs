use std::env;

use env_logger::{Builder as LoggerBuilder, Target};
// use jieba_rs::Jieba;
// use simsearch::{SearchOptions, SimSearch};
// use strsim::damerau_levenshtein as a;
// use textdistance::nstr::damerau_levenshtein;
use tokio::runtime::Builder;
// use triple_accel::levenshtein::levenshtein_simd_k;

use dialogflow::web::server::start_app;

fn main() {
    // dialogflow::web::t1();
    unsafe {
        env::set_var("RUST_LOG", "INFO");
    }
    let mut builder = LoggerBuilder::from_default_env();
    builder
        .target(Target::Stdout)
        .format_module_path(false)
        .format_target(false)
        .format_indent(None);
    builder.init();

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("dialogflowchatbot")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    // let (sender, recv) = tokio::sync::oneshot::channel::<()>();
    // runtime.spawn(dialogflow::web::clean_expired_session(recv));
    runtime.block_on(start_app());
}
