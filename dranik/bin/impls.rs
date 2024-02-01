mod telemetry;
mod gamepad;
mod runtime;

/// Internal struct that is used to hold the robot config.
/// 
/// This contains the default config.
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub struct __dranik_config;
impl ::dranik_api::RobotConfig for __dranik_config {
    // type Runtime = self::runtime::Runtime;
    type Args = (self::runtime::py::DranikRuntime, );
    fn python_preload(&self) {}
    fn make_main_function_args<'a>(&self, _py: pyo3::Python<'a>) -> (Self::Args, Option<&'a pyo3::types::PyDict>) {
        // ((self::runtime::py::DranikRuntime::new(py), ), None)
        todo!("make_main_function_args")
    }
}