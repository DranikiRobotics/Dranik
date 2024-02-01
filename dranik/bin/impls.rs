use dranik_api::threadsafe::*;
use dranik_api::prelude::*;
use dranik_api::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryImpl;
impl Telemetry for TelemetryImpl {
    fn debug<T: core::fmt::Debug>(&self, message: T) {
        todo!()
    }
    fn send<T: core::fmt::Display>(&self, message: T) {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GamepadImpl;
impl Gamepad for GamepadImpl {
    fn dpad(&self) -> Result<gamepad::GamepadDpad> {
        todo!()
    }
    fn left_stick(&self) -> Result<gamepad::GamepadStick> {
        todo!()
    }
    fn right_stick(&self) -> Result<gamepad::GamepadStick> {
        todo!()
    }
    fn left_trigger(&self) -> Result<f64> {
        todo!()
    }
    fn right_trigger(&self) -> Result<f64> {
        todo!()
    }
    fn a(&self) -> Result<bool> {
        todo!()
    }
    fn b(&self) -> Result<bool> {
        todo!()
    }
    fn x(&self) -> Result<bool> {
        todo!()
    }
    fn y(&self) -> Result<bool> {
        todo!()
    }
    fn left_bumper(&self) -> Result<bool> {
        todo!()
    }
    fn right_bumper(&self) -> Result<bool> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpImpl {
    pub(crate) telemetry: ThreadSafe<TelemetryImpl>,
    pub(crate) gamepad: ThreadSafe<GamepadImpl>,
    pub(crate) start_time: std::time::Instant,
    pub(crate) running: ThreadSafeBool
}
impl Op for OpImpl {
    type TelemetryImpl = TelemetryImpl;
    type GamepadImpl = GamepadImpl;
    fn running(&self) -> bool {
        match self.running.get() {
            Ok(running) => **running,
            Err(_) => false
        }
    }
    fn running_result(&self) -> Result<bool> {
        match self.running.get() {
            Err(e) => Err(Error::new(e)),
            Ok(running) => Ok(**running)
        }
    }
    #[inline]
    fn telemetry(&self) -> threadsafe::GetResult<'_, Self::TelemetryImpl> {
        self.telemetry.get()
    }
    #[inline]
    fn get_telemetry(&self) -> threadsafe::ThreadSafe<Self::TelemetryImpl> {
        self.telemetry.clone()
    }
    #[inline]
    fn gamepad(&self) -> threadsafe::GetResult<'_, Self::GamepadImpl> {
        self.gamepad.get()
    }
    #[inline]
    fn get_gamepad(&self) -> threadsafe::ThreadSafe<Self::GamepadImpl> {
        self.gamepad.clone()
    }
    #[inline]
    fn get_start_time(&self) -> std::time::Instant {
        self.start_time
    }
}
