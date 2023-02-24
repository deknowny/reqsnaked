use pyo3::PyResult;

pub trait ToNative {
    type Native;
    fn to_native(&self) -> PyResult<Self::Native>;
}
