use pyo3::prelude::*;

#[pyclass]
#[doc(hidden)]
struct DranikRuntime(String);

#[pymodule]
pub(crate) fn dranik(py: Python<'_>, m: &PyModule) -> pyo3::PyResult<()> {

}
