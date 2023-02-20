use pyo3::prelude::*;

pub mod aio;
pub mod json;
pub mod primitives;
pub mod py2rs;
pub mod request;

#[pymodule]
fn reqsnaked(py: Python, module: &PyModule) -> PyResult<()> {
    aio::init_module(py, module, module)?;
    primitives::init_module(py, module, module)?;
    request::init_module(py, module, module)?;
    Ok(())
}
