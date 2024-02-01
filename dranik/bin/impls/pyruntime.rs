use pyo3::types::PyTuple;
use pyo3::prelude::*;

use dranik_api::{Telemetry, Gamepad};

/// Internal struct that is used to interact with the Python runtime.
#[pyclass]
#[doc(hidden)]
#[derive(Debug)]
pub struct DranikRuntime(super::Runtime);

#[pymethods]
impl DranikRuntime {
    #[pyo3(signature = (*message))]
    fn log(&self, message: &PyTuple) {
        let mut msg = String::new();
        for item in message.iter() {
            let s = item.extract::<String>().unwrap_or(String::new());
            msg.push_str(&s);
        }
        self.0.log(msg);
    }
    #[pyo3(signature = (*message))]
    fn debug(&self, message: &PyTuple) {
        let mut msg = String::new();
        for item in message.iter() {
            let s = item.extract::<String>().unwrap_or(String::new());
            msg.push_str(&s);
        }
        self.0.debug(msg);
    }
    #[getter]
    #[inline(always)]
    fn dpad_up(&self) -> bool {
        self.0.dpad_up()
    }
    #[getter]
    #[inline(always)]
    fn dpad_down(&self) -> bool {
        self.0.dpad_down()
    }
    #[getter]
    #[inline(always)]
    fn dpad_left(&self) -> bool {
        self.0.dpad_left()
    }
    #[getter]
    #[inline(always)]
    fn dpad_right(&self) -> bool {
        self.0.dpad_right()
    }
    #[getter]
    #[inline(always)]
    fn lstick_x(&self) -> f32 {
        self.0.lstick_x()
    }
    #[getter]
    #[inline(always)]
    fn lstick_y(&self) -> f32 {
        self.0.lstick_y()
    }
    #[getter]
    #[inline(always)]
    fn lstick(&self) -> bool {
        self.0.lstick()
    }
    #[getter]
    #[inline(always)]
    fn rstick_x(&self) -> f32 {
        self.0.rstick_x()
    }
    #[getter]
    #[inline(always)]
    fn rstick_y(&self) -> f32 {
        self.0.rstick_y()
    }
    #[getter]
    #[inline(always)]
    fn rstick(&self) -> bool {
        self.0.rstick()
    }
    #[getter]
    #[inline(always)]
    fn ltrigger(&self) -> f32 {
        self.0.ltrigger()
    }
    #[getter]
    #[inline(always)]
    fn rtrigger(&self) -> f32 {
        self.0.rtrigger()
    }
    #[getter]
    #[inline(always)]
    fn a(&self) -> bool {
        self.0.a()
    }
    #[getter]
    #[inline(always)]
    fn b(&self) -> bool {
        self.0.b()
    }
    #[getter]
    #[inline(always)]
    fn x(&self) -> bool {
        self.0.x()
    }
    #[getter]
    #[inline(always)]
    fn y(&self) -> bool {
        self.0.y()
    }
    #[getter]
    #[inline(always)]
    fn lbumper(&self) -> bool {
        self.0.lbumper()
    }
    #[getter]
    #[inline(always)]
    fn rbumper(&self) -> bool {
        self.0.rbumper()
    }
}
