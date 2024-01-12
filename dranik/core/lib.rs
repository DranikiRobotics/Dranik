//! The core library for the Dranik robot.

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[cfg(not(feature = "reveal_modules"))]
mod threadsafe;
#[cfg(feature = "reveal_modules")]
pub mod threadsafe;
#[cfg(not(feature = "reveal_modules"))]
mod gamepad;
#[cfg(feature = "reveal_modules")]
pub mod gamepad;

mod telemetry;
mod config;
mod error;
mod op;

/// The prelude for the `dranikcore` crate.
/// 
/// This prelude re-exports all of the important types and traits
pub mod prelude {
    pub use super::config::RobotConfig;
    pub use super::gamepad::Gamepad;
    pub use super::telemetry::Telemetry;
    pub use super::threadsafe::ThreadSafeError;
    pub use super::op::Op;
}

pub use error::{HardwareError, Result, IO_OK};
