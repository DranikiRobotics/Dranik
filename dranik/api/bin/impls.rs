use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryImpl;
impl Telemetry for TelemetryImpl {
    fn debug<T: std::fmt::Debug>(&self, message: T) {
        todo!()
    }
    fn send<T: std::fmt::Display>(&self, message: T) {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GamepadImpl;
impl Gamepad for GamepadImpl {
    fn dpad(&self) -> crate::Result<crate::gamepad::GamepadDpad> {
        todo!()
    }
    fn left_stick(&self) -> crate::Result<crate::gamepad::GamepadStick> {
        todo!()
    }
    fn right_stick(&self) -> crate::Result<crate::gamepad::GamepadStick> {
        todo!()
    }
    fn left_trigger(&self) -> crate::Result<l2math::Float64> {
        todo!()
    }
    fn right_trigger(&self) -> crate::Result<l2math::Float64> {
        todo!()
    }
    fn a(&self) -> crate::Result<bool> {
        todo!()
    }
    fn b(&self) -> crate::Result<bool> {
        todo!()
    }
    fn x(&self) -> crate::Result<bool> {
        todo!()
    }
    fn y(&self) -> crate::Result<bool> {
        todo!()
    }
    fn left_bumper(&self) -> crate::Result<bool> {
        todo!()
    }
    fn right_bumper(&self) -> crate::Result<bool> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpImpl;
impl Op for OpImpl {
    type TelemetryImpl = TelemetryImpl;
    type GamepadImpl = GamepadImpl;
    fn running(&self) -> bool {
        todo!()
    }
    fn running_result(&self) -> crate::Result<bool> {
        todo!()
    }
    fn telemetry(&self) -> crate::threadsafe::GetResult<'_, Self::TelemetryImpl> {
        todo!()
    }
    fn get_telemetry(&self) -> crate::threadsafe::ThreadSafe<Self::TelemetryImpl> {
        todo!()
    }
    fn gamepad(&self) -> crate::threadsafe::GetResult<'_, Self::GamepadImpl> {
        todo!()
    }
    fn get_gamepad(&self) -> crate::threadsafe::ThreadSafe<Self::GamepadImpl> {
        todo!()
    }
    fn get_start_time(&self) -> std::time::Instant {
        todo!()
    }
}
