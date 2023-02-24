use pyo3::{prelude::*, types::PyBytes};

#[derive(Clone)]
#[pyclass]
pub struct Bytes(pub bytes::Bytes);

#[pymethods]
impl Bytes {
    pub fn as_bytes<'rt>(&'rt self, py: Python<'rt>) -> &'rt PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }
}
