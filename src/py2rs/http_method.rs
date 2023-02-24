use std::str::FromStr;

use pyo3::prelude::*;

use crate::exceptions::HTTPMethodParseError;

use super::base::ToNative;

#[derive(FromPyObject)]
pub enum HTTPMethod {
    String(String),
}

impl ToNative for HTTPMethod {
    type Native = reqwest::Method;

    fn to_native(&self) -> PyResult<Self::Native> {
        match self {
            Self::String(value) => {
                let parsing_result = reqwest::Method::from_str(&value);
                parsing_result.map_err(|e| {
                    HTTPMethodParseError::new_err(format!("Cannot parse URL: {:?}", e))
                })
            }
        }
    }
}
