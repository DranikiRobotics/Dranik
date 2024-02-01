//! Dranik, a framework for creating robots.
//! 
//! This crate is the main crate for the Dranik framework.
//! It allows you to simply create a robot using the Dranik framework.
//! 
//! However, note that the robot's code is written in python, rather than rust.
//! 
//! ## Example
//! 
//! ```python
//! from dranik.api import DranikRuntime
//! 
//! def main(r: DranikRuntime):
//!     r.log("Hello, world!")
//! ```

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

/// Helps with loading robot configurations.
/// 
/// ## Example using the default config
/// 
/// ```rust
/// dranik::use_config!();
/// // or
/// // dranik::use_config!(dranik);
/// dranik::main!();
/// ```
/// 
/// ## Example using ARC
/// 
/// ```rust
/// dranik::use_config!(arc);
/// dranik::main!();
/// ```
/// 
/// This macro is used to help with loading robot configurations.
/// 
/// It is guaranteed to be stable and it's API will not change.
/// (Without a major version bump)
#[macro_export(local_inner_macros)]
macro_rules! load_config {
    () => ( $crate::load_config!($crate); );
    ($namespace: path) => (
        use $namespace::{__dranik_config as __DranikRobotConfig};
    );
}

/// Creates the main function for the robot.
/// 
/// ## Example
/// 
/// ```rust
/// dranik::use_config!();
/// dranik::main!();
/// ```
/// 
/// This macro is used to create the main function for the robot.
/// It is guaranteed to be stable and it's API will not change.
/// (Without a major version bump)
/// 
/// The reason this macro exists is because the main function
/// is not guaranteed to be stable and may change at any time.
/// That mostly includes the generic parameters.
#[macro_export(local_inner_macros)]
macro_rules! main {
    () => (fn main() { $crate::main!(@); });
    (@) => ($crate::__main::<__DranikRobotConfig>(););
}

#[doc(hidden)]
pub use dranik_bin::{__dranik_config, __main};
