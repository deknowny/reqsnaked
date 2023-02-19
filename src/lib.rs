use pyo3::prelude::*;

pub mod aio;
pub mod primitives;
pub mod json;

#[pymodule]
fn reqsnaked(py: Python, module: &PyModule) -> PyResult<()> {
    aio::init_module(py, module)?;
    primitives::init_module(py, module)?;
    Ok(())
}
