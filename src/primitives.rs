use std::str::FromStr;

use pyo3::prelude::*;

#[derive(FromPyObject)]
pub enum HTTPMethod {
    String(String),
}

impl HTTPMethod {
    pub fn to_native(&self) -> PyResult<reqwest::Method> {
        match self {
            Self::String(value) => {
                let parsing_result = reqwest::Method::from_str(&value);
                // TODO: Py error
                Ok(parsing_result.unwrap())
            }
        }
    }
}

#[derive(FromPyObject)]
pub enum URL {
    String(String),
}

impl URL {
    pub fn to_native(&self) -> PyResult<url::Url> {
        match self {
            Self::String(value) => {
                let parsing_result = url::Url::parse(&value);
                // TODO: Py error
                Ok(parsing_result.unwrap())
            }
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct StatusCode(pub reqwest::StatusCode);

#[pymethods]
impl StatusCode {
    #[getter]
    pub fn code(&self) -> u16 {
        self.0.as_u16()
    }
}

#[derive(Clone)]
#[pyclass]
pub struct BasicAuth {
    #[pyo3(get, set)]
    pub username: String,

    #[pyo3(get, set)]
    pub password: Option<String>,
}

#[pymethods]
impl BasicAuth {
    #[new]
    pub fn new(username: String, password: Option<String>) -> BasicAuth {
        BasicAuth { username, password }
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "primitives")?;
    submod.add_class::<StatusCode>()?;
    submod.add_class::<BasicAuth>()?;

    library.add_class::<StatusCode>()?;
    library.add_class::<BasicAuth>()?;

    parent_module.add_submodule(submod)?;
    Ok(())
}
