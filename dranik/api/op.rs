use super::threadsafe::*;
use super::prelude::*;
use super::*;

/// An `Op`eration that is going to be run by the robot

pub trait Op: Clone + PartialEq + Send + Sync {
    /// The telemetry type of the op mode
    type TelemetryImpl: Telemetry;
    /// The gamepad type of the op mode
    type GamepadImpl: Gamepad;

    /// Returns if the op mode is running
    /// 
    /// If the mutex is poisoned, this will return false
    fn running(&self) -> bool;
    /// Returns if the op mode is running
    /// 
    /// The result is wrapped in a `Result` because
    /// internally, the value is wrapped in a `Mutex`
    /// 
    /// If the [`Mutex`] is poisoned, this will return an error
    /// 
    /// [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
    fn running_result(&self) -> Result<bool>;
    /// Returns the [`Telemetry`] of the op mode.
    /// 
    /// It is mostly used for sending log messages to the driver control station.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    fn telemetry(&self) -> GetResult<'_, Self::TelemetryImpl>;
    /// Returns a cloned version of the [`Telemetry`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    fn get_telemetry(&self) -> ThreadSafe<Self::TelemetryImpl>;
    /// Returns the [`Gamepad`] of the op mode.
    /// 
    /// Used for controlling the in TeleOp Modes.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    fn gamepad(&self) -> GetResult<'_, Self::GamepadImpl>;
    /// Returns a cloned version of the [`Gamepad`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    fn get_gamepad(&self) -> ThreadSafe<Self::GamepadImpl>;
    /// Returns the amount of time that the op mode has been running for
    fn get_start_time(&self) -> std::time::Instant;
    /// Returns the amount of time that the op mode has been running for
    #[inline(always)]
    fn get_running_for(&self) -> core::time::Duration {
        std::time::Instant::now() - self.get_start_time()
    }
}
