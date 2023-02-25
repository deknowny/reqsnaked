use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict},
};

#[derive(Clone)]
#[pyclass]
pub struct HeaderMap(pub reqwest::header::HeaderMap);

#[pymethods]
impl HeaderMap {
    pub fn to_dict<'rt>(&self, py: Python<'rt>) -> PyResult<&'rt PyDict> {
        let new_dict = PyDict::new(py);
        for (header, value) in &self.0 {
            new_dict.set_item(header.as_str(), PyBytes::new(py, value.as_ref()))?;
        }
        Ok(new_dict)
    }

    fn __getitem__<'rt>(&'rt self, key: &str) -> PyResult<Option<&'rt [u8]>> {
        Ok(self.0.get(key).and_then(|v| Some(v.as_ref())))
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "header_map")?;
    submod.add_class::<HeaderMap>()?;
    library.add_class::<HeaderMap>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
