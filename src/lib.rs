use pyo3::prelude::*;

pub mod aio;
pub mod exceptions;
pub mod json;
pub mod py2rs;
pub mod rs2py;

#[pymodule]
fn reqsnaked(py: Python, module: &PyModule) -> PyResult<()> {
    aio::init_module(py, module, module)?;
    rs2py::init_module(py, module, module)?;
    Ok(())
}
