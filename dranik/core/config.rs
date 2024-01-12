use crate::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict};

use pyo3::Python;

/// This trait is used to completely configure the robot.
pub trait RobotConfig {
    /// This function is called before the python interpreter is initialized
    /// 
    /// This is useful for loading rust libraries that will then be used by python.
    /// In fact, this is exactly how ARC is loaded.
    #[inline(always)]
    fn python_preload(&self) {}
    /// The type that will be used for telemetry
    type TelemetryImpl: Telemetry;
    /// The type that will be used for gamepad
    type GamepadImpl: Gamepad;
    /// The type that will be used for op
    type OpImpl: Op<Self::TelemetryImpl, Self::GamepadImpl>;
    /// This function is called to build the arguments that will be passed to the python main function
    #[inline(always)]
    fn build_python_main_function_args<'a>(
        &self, _py: &Python<'_>, _op: &Self::OpImpl
    ) -> (impl IntoPy<Py<PyTuple>>, Option<&'a PyDict>) {
        ((), None)
    }
}