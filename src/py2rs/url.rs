use pyo3::prelude::*;

use crate::exceptions::URLParseError;

#[derive(FromPyObject)]
pub enum URL {
    String(String),
}

impl URL {
    pub fn to_native(&self) -> PyResult<url::Url> {
        match self {
            Self::String(value) => url::Url::parse(&value)
                .map_err(|e| URLParseError::new_err(format!("Cannot parse url: {:?}", e))),
        }
    }
}
