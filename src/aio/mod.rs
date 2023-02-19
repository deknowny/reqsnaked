use pyo3::prelude::*;

pub mod client;
pub mod response;
pub mod request;


pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "aio")?;
    client::init_module(py, submod)?;
    request::init_module(py, submod)?;
    response::init_module(py, submod)?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
