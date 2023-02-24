use pyo3::prelude::*;

pub mod form;
pub mod part;


pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "aio")?;
    form::init_module(py, submod, library)?;
    part::init_module(py, submod, library)?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
