//! The main crate for the dranik library.
//! 

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

/// Helps with loading robot configurations.
/// 
/// ## Example using the default config
/// 
/// ```rust
/// dranik::use_config!();
/// dranik::main!();
/// ```
/// 
/// ## Example using ARC
/// 
/// ```rust
/// dranik::use_config!(arc);
/// dranik::main!();
/// ```
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
/// 
/// The reason this macro exists is because the main function
/// is not guaranteed to be stable and may change at any time.
/// That mostly includes the generic parameters.
#[macro_export(local_inner_macros)]
macro_rules! main {
    () => ($crate::__main::<__DranikRobotConfig>(););
}

#[doc(hidden)]
pub use dranik_bin::{__dranik_config, __main};
