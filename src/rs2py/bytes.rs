use pyo3::{prelude::*, types::PyBytes};


#[derive(Clone)]
#[pyclass(module = "reqsnaked")]
pub struct Bytes(pub bytes::Bytes);

#[pymethods]
impl Bytes {
    pub fn as_bytes<'rt>(&'rt self, py: Python<'rt>) -> &'rt PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "bytes")?;
    submod.add_class::<Bytes>()?;
    library.add_class::<Bytes>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
