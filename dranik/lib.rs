#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

mod internals;

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
macro_rules! use_config {
    () => ( $crate::use_config!($crate); );
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
    () => ( fn main() {
        $crate::main::<__DranikRobotConfig>();
    } );
}

pub use dranikcore::prelude::RobotConfig;

/// This is the actual main function that is called by the robot.
/// 
/// It isn't recommended to call this function directly.
/// However, if you do, be careful as this function is not
/// guaranteed to be stable and may change at any time.
/// See [`main!`] for more information.
/// 
/// ## Recommended Usage
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
/// 
/// [`main!`]: crate::main!
pub fn main<C: RobotConfig + 'static>() {
    internals::main::<C>();
}

#[doc(hidden)]
pub use internals::__dranik_config;
