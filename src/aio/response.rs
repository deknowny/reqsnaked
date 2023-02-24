use pyo3::prelude::*;

use crate::json::{LazyJSON, PySerde};
use crate::{primitives, rs2py};
use pyo3::exceptions::PyRuntimeError;

#[pyclass]
pub struct AsyncStream {
    pub response: std::sync::Arc<tokio::sync::Mutex<reqwest::Response>>,
}

#[pymethods]
impl AsyncStream {
    pub fn gnaw<'rt>(slf: PyRef<'rt, Self>, py: Python<'rt>) -> PyResult<&'rt pyo3::PyAny> {
        let response = slf.response.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            // TODO:
            Ok(response
                .lock()
                .await
                .chunk()
                .await
                .unwrap()
                .and_then(|bytes| Some(rs2py::bytes::Bytes(bytes))))
        })
    }
}

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
                // TODO:
                let parsed_body: PySerde = response.json().await.unwrap();
                Ok(LazyJSON(parsed_body))
            });
        }
        Err(PyRuntimeError::new_err(
            "Response body has been already read",
        ))
    }

    pub fn read<'rt>(&'rt self, py: Python<'rt>) -> PyResult<&'rt pyo3::PyAny> {
        if let Some(response) = self.response.borrow_mut().take() {
            return pyo3_asyncio::tokio::future_into_py(py, async move {
                // TODO:
                let parsed_body: PySerde = response.json().await.unwrap();
                Ok(LazyJSON(parsed_body))
            });
        }
        Err(PyRuntimeError::new_err(
            "Response body has been already read",
        ))
    }

    pub fn to_stream(&self) -> PyResult<AsyncStream> {
        if let Some(response) = self.response.borrow_mut().take() {
            return Ok(AsyncStream {
                response: std::sync::Arc::new(tokio::sync::Mutex::new(response)),
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
    submod.add_class::<AsyncStream>()?;
    library.add_class::<AsyncStream>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
