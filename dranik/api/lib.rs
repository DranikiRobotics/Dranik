//! The API for the dranik robot.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(error_in_core))]

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

mod gamepad;
mod telemetry;
mod config;
mod error;

pub use error::{Error, Result, IO_OK};
pub use gamepad::Gamepad;
pub use telemetry::Telemetry;
#[cfg(feature = "config")]
pub use config::RobotConfig;
#[cfg(feature = "config")]
pub use telemetry::TelemetryMessage;

/// The runtime for the dranik robot.
/// 
/// This trait is a combination of the `Gamepad` and `Telemetry` traits.
/// 
/// This trait is not meant to be implemented by the user.
/// 
/// # IF YOU ARE IMPLEMENTING THIS TRAIT, YOU ARE DOING SOMETHING WRONG.
#[cfg_attr(feature = "config", doc = "
Note for internal's developers:

Points to `.dranik.api.DranikRuntime` in `.dranik.api`")]
pub trait DranikRuntime: Gamepad + Telemetry {
    
}
