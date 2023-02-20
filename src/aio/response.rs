use pyo3::prelude::*;

use crate::json::{LazyJSON, PySerde};
use crate::primitives;
use pyo3::exceptions::PyRuntimeError;

#[pyclass]
pub struct AsyncResponse {
    pub response: std::cell::RefCell<Option<reqwest::Response>>,
    pub status: reqwest::StatusCode,
}

#[pymethods]
impl AsyncResponse {
    #[getter]
    pub fn status(&self) -> primitives::StatusCode {
        primitives::StatusCode(self.status)
    }

    pub fn json<'rt>(&self, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        if let Some(response) = self.response.borrow_mut().take() {
            return pyo3_asyncio::tokio::future_into_py(py, async move {
                let parsed_body: PySerde = response.json().await.unwrap();
                Ok(LazyJSON(parsed_body))
            });
        }
        Err(PyRuntimeError::new_err(
            "Response body has been already read",
        ))
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "response")?;
    submod.add_class::<AsyncResponse>()?;
    library.add_class::<AsyncResponse>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
