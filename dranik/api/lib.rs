//! The API for the dranik robot.

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
#[cfg(feature = "reveal_modules")]
pub mod threadsafe;
#[doc(hidden)]
#[cfg(not(feature = "reveal_modules"))]
pub(crate) mod threadsafe;
#[doc(hidden)]
#[cfg(feature = "reveal_modules")]
pub mod gamepad;
#[doc(hidden)]
#[cfg(not(feature = "reveal_modules"))]
pub(crate) mod gamepad;
mod telemetry;
mod config;
mod error;
mod op;

mod __init__;

/// Common traits and types used by the robot.
pub mod prelude {
    pub use super::config::RobotConfig;
    pub use super::gamepad::Gamepad;
    pub use super::telemetry::Telemetry;
    pub use super::threadsafe::ThreadSafeError;
    pub use super::op::Op;
}

pub use error::{Error, Result, IO_OK};
