use pyo3::prelude::*;

pub mod client;
pub mod response;
pub mod multipart;

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "aio")?;
    client::init_module(py, submod, library)?;
    response::init_module(py, submod, library)?;
    multipart::init_module(py, submod, library)?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
