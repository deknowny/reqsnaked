use pyo3::prelude::*;

use crate::exceptions::{wrap_reqwest_error, BorrowingError, RACE_CONDITION_ERROR_MSG};
use crate::json::{LazyJSON, PySerde};
use crate::rs2py;
use pyo3::exceptions::PyRuntimeError;

#[pyclass]
pub struct Stream {
    pub response: std::sync::Arc<tokio::sync::Mutex<reqwest::Response>>,
}

#[pymethods]
impl Stream {
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
    pub headers: reqwest::header::HeaderMap,
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

    #[getter]
    pub fn headers(&self) -> rs2py::header_map::HeaderMap {
        rs2py::header_map::HeaderMap(self.headers.clone())
    }

    pub fn json<'rt>(&self, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        match self.response.take() {
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
        match self
            .response
            .try_borrow_mut()
            .map_err(|_| BorrowingError::new_err(RACE_CONDITION_ERROR_MSG))?
            .take()
        {
            Some(response) => pyo3_asyncio::tokio::future_into_py(py, async move {
                match response.bytes().await {
                    Ok(body) => Ok(rs2py::bytes::Bytes(body)),
                    Err(err) => Err(wrap_reqwest_error(err)),
                }
            }),
            None => Err(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG)),
        }
    }

    pub fn to_stream(&self) -> PyResult<Stream> {
        match self
            .response
            .try_borrow_mut()
            .map_err(|_| BorrowingError::new_err(RACE_CONDITION_ERROR_MSG))?
            .take()
        {
            Some(response) => Ok(Stream {
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
    submod.add_class::<Stream>()?;
    library.add_class::<Stream>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
