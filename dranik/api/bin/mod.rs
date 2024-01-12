use crate::prelude::*;

mod impls;

pub fn main<C: RobotConfig + 'static>() {
    println!("Hello, world!");
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