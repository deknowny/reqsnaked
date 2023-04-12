use chardet::detect;
use encoding_rs::{Encoding, UTF_8};
use pyo3::{prelude::*, types::{PyBytes, PyString}};


#[derive(Clone)]
#[pyclass(module = "reqsnaked")]
pub struct Bytes(pub bytes::Bytes);

#[pymethods]
impl Bytes {
    pub fn as_bytes<'rt>(&'rt self, py: Python<'rt>) -> &'rt PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }
    pub fn decode<'rt>(&'rt self, py: Python<'rt>) -> &'rt PyString {
        let encoding = detect(self.0.as_ref()).0;

        let encoding = Encoding::for_label(encoding.as_bytes()).unwrap_or(UTF_8);     
        PyString::new(py, &encoding.decode(self.0.as_ref()).0)
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "bytes")?;
    submod.add_class::<Bytes>()?;
    library.add_class::<Bytes>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
