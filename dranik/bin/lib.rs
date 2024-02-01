use dranik_api::prelude::*;

mod impls;

use std::sync::atomic::{AtomicU8, Ordering};
use tokio::runtime::Builder;

use axum::{routing::get, Router};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

static ID: AtomicU8 = AtomicU8::new(0);

pub fn __main<C: RobotConfig + 'static>() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name_fn(|| {
            let id = ID.fetch_add(1, Ordering::SeqCst);
            format!("Dranik worker#{}", id)
        })
        .build()
        .expect("Failed to initialize runtime");

    if let Err(e) = runtime.block_on(main()) {
        eprintln!("Dranik runtime exited with error: {}", e);
        std::process::exit(1);
    }
    println!("Dranik runtime exited");
    std::process::exit(0);
}

async fn main() -> ::dranik_api::Result {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with some routes
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true));
        // .route("/ws", get(ws::ws_handler))

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on https://{}", listener.local_addr().unwrap());
    axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

/// Internal struct that is used to hold the robot config.
/// 
/// This contains the default config.
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub struct __dranik_config;
impl RobotConfig for __dranik_config {
    type TelemetryImpl = impls::TelemetryImpl;
    type GamepadImpl = impls::GamepadImpl;
    type OpImpl = impls::OpImpl;
}
