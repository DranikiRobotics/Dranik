use crate::threadsafe::*;
use crate::prelude::*;
use crate::*;

/// An `Op`eration that is going to be run by the robot

pub trait Op<T, G> where
    T: Telemetry,
    G: Gamepad,
{
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
    fn telemetry(&self) -> GetResult<'_, T>;
    /// Returns a cloned version of the [`Telemetry`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    fn get_telemetry(&self) -> ThreadSafe<T>;
    /// Returns the [`Gamepad`] of the op mode.
    /// 
    /// Used for controlling the in TeleOp Modes.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    fn gamepad(&self) -> GetResult<'_, G>;
    /// Returns a cloned version of the [`Gamepad`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    fn get_gamepad(&self) -> ThreadSafe<G>;
    /// Returns the amount of time that the op mode has been running for
    fn get_start_time(&self) -> std::time::Instant;
    /// Returns the amount of time that the op mode has been running for
    #[inline(always)]
    fn get_running_for(&self) -> core::time::Duration {
        std::time::Instant::now() - self.get_start_time()
    }
}

impl Op<(), ()> for () {
    #[inline(always)]
    fn running(&self) -> bool {
        false
    }
    #[inline(always)]
    fn running_result(&self) -> Result<bool> {
        Err(HardwareError::MethodNotImplemented)
    }
    #[inline(always)]
    fn telemetry(&self) -> GetResult<'_, ()> {
        Err(HardwareError::MethodNotImplemented.as_str())
    }
    #[inline(always)]
    fn get_telemetry(&self) -> ThreadSafe<()> {
        unreachable!("This should never be called")
    }
    #[inline(always)]
    fn gamepad(&self) -> GetResult<'_, ()> {
        Err(HardwareError::MethodNotImplemented.as_str())
    }
    #[inline(always)]
    fn get_gamepad(&self) -> ThreadSafe<()> {
        unreachable!("This should never be called")
    }
    #[inline(always)]
    fn get_start_time(&self) -> std::time::Instant {
        unreachable!("This should never be called")
    }
}
