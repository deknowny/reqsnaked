use std::str::FromStr;

use pyo3::prelude::*;

use crate::exceptions::HTTPMethodParseError;

use super::base::ToNative;



pub struct PyHTTPMethodEnum(reqwest::Method);

impl FromPyObject<'_> for PyHTTPMethodEnum {
    fn extract<'rt>(ob: &'rt PyAny) -> PyResult<Self> {
        let code: &str = ob.getattr("value")?.extract()?;
        let method = reqwest::Method::from_str(code).map_err(|e| HTTPMethodParseError::new_err(format!("Cannot parse HTTP methods: {:?}", e)))?;
        Ok(Self(method))
    }
}



#[derive(FromPyObject)]
pub enum HTTPMethod {
    String(String),
    HTTPMethodEnum(PyHTTPMethodEnum)
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
            },
            Self::HTTPMethodEnum(method) => Ok(method.0.clone())
        }
    }
}
