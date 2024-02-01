//! The crate for the binary of the robot.
//! 
//! This crate doesn't have any public API, and thus should not be used directly.
//! 
//! If you want to use the robot, you should use the `dranik` crate instead.
//! 
//! ## Example
//! 
//! ```rust
//! dranik::use_config!();
//! dranik::main!();
//! ```

use ::dranik_api::RobotConfig;

mod threadsafe;
#[path = "main.rs"]
mod main;
mod impls;
mod ws;

use std::sync::atomic::{AtomicU8, Ordering};
use tokio::runtime::Builder;

static ID: AtomicU8 = AtomicU8::new(0);

#[doc(hidden)]
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

    if let Err(e) = runtime.block_on(main::main()) {
        eprintln!("Dranik runtime exited with error: {}", e);
        std::process::exit(1);
    }
    println!("Dranik runtime exited");
    std::process::exit(0);
}

#[doc(hidden)]
pub use self::impls::__dranik_config;
