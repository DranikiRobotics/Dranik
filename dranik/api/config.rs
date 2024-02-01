use pyo3::types::{PyDict, PyTuple};
use pyo3::*;

/// This trait is used to completely configure the robot.
pub trait RobotConfig {
    /// This function is called before the python interpreter is initialized
    /// 
    /// This is useful for loading rust libraries that will then be used by python.
    /// In fact, this is exactly how ARC is loaded.
    fn python_preload(&self);
    /// The type of the main function arguments.
    type Args: IntoPy<Py<PyTuple>>;
    /// This function is used to create the main function arguments.
    fn make_main_function_args<'a>(&self, py: Python<'a>) -> (Self::Args, Option<&'a PyDict>);
}