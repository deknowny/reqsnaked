use pyo3::prelude::*;

pub mod bytes;
pub mod http_version;

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "rs2py")?;
    bytes::init_module(py, submod, library)?;
    http_version::init_module(py, submod, library)?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
