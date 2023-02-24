use pyo3::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass]
pub enum HTTPVersion {
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
}

impl From<reqwest::Version> for HTTPVersion {
    fn from(value: reqwest::Version) -> Self {
        match value {
            reqwest::Version::HTTP_09 => Self::HTTP_09,
            reqwest::Version::HTTP_10 => Self::HTTP_10,
            reqwest::Version::HTTP_11 => Self::HTTP_11,
            reqwest::Version::HTTP_2 => Self::HTTP_2,
            reqwest::Version::HTTP_3 => Self::HTTP_3,
            _ => unreachable!(),
        }
    }
}

#[pymethods]
impl HTTPVersion {
    pub fn to_string(&self) -> String {
        match self {
            Self::HTTP_09 => "HTTP/0.9",
            Self::HTTP_10 => "HTTP/1.0",
            Self::HTTP_11 => "HTTP/1.1",
            Self::HTTP_2 => "HTTP/2.0",
            Self::HTTP_3 => "HTTP/3.0",
        }
        .to_string()
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "http_version")?;
    submod.add_class::<HTTPVersion>()?;
    library.add_class::<HTTPVersion>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
