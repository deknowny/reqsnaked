use pyo3::prelude::*;

use crate::exceptions::{wrap_reqwest_error, BorrowingError, RACE_CONDITION_ERROR_MSG};
use crate::json::{LazyJSON, PySerde};
use crate::rs2py;
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
            let chunk = response
                .lock()
                .await
                .chunk()
                .await
                .map_err(wrap_reqwest_error)?;
            Ok(chunk.and_then(|bytes| Some(rs2py::bytes::Bytes(bytes))))
        })
    }
}

#[pyclass]
pub struct AsyncResponse {
    pub response: std::cell::RefCell<Option<reqwest::Response>>,
    pub status: reqwest::StatusCode,
    pub version: reqwest::Version,
}

#[pymethods]
impl AsyncResponse {
    #[getter]
    pub fn status<'rt>(&'rt self, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        let http_module = py.import("http")?;
        let http_enum = http_module.getattr("HTTPStatus")?;
        http_enum.call1((self.status.as_u16(),))
    }

    #[getter]
    pub fn version(&self) -> rs2py::http_version::HTTPVersion {
        rs2py::http_version::HTTPVersion::from(self.version)
    }

    pub fn json<'rt>(&self, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        match self.response.borrow_mut().take() {
            Some(response) => pyo3_asyncio::tokio::future_into_py(py, async move {
                match response.json::<PySerde>().await {
                    Ok(body) => Ok(LazyJSON(body)),
                    Err(err) => Err(wrap_reqwest_error(err)),
                }
            }),
            None => Err(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG)),
        }
    }

    pub fn read<'rt>(&'rt self, py: Python<'rt>) -> PyResult<&'rt pyo3::PyAny> {
        match self.response.borrow_mut().take() {
            Some(response) => pyo3_asyncio::tokio::future_into_py(py, async move {
                match response.bytes().await {
                    Ok(body) => Ok(rs2py::bytes::Bytes(body)),
                    Err(err) => Err(wrap_reqwest_error(err)),
                }
            }),
            None => Err(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG)),
        }
    }

    pub fn to_stream(&self) -> PyResult<AsyncStream> {
        match self.response.borrow_mut().take() {
            Some(response) => Ok(AsyncStream {
                response: std::sync::Arc::new(tokio::sync::Mutex::new(response)),
            }),
            None => Err(PyRuntimeError::new_err(
                "Response body has been already read",
            )),
        }
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
