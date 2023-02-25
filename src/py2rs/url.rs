use pyo3::prelude::*;

use crate::exceptions::URLParseError;

pub struct PyUrllibParseResult(url::Url);

impl FromPyObject<'_> for PyUrllibParseResult {
    fn extract<'rt>(ob: &'rt PyAny) -> PyResult<Self> {
        let stringified_url: &str = ob.call_method0("geturl")?.extract()?;
        let rs_url: url::Url = stringified_url
            .parse()
            .map_err(|e| URLParseError::new_err(format!("Cannot parse URL: {:?}", e)))?;
        Ok(Self(rs_url))
    }
}

#[derive(FromPyObject)]
pub enum URL {
    String(String),
    UrllibParseResult(PyUrllibParseResult),
}

impl URL {
    pub fn to_native(&self) -> PyResult<url::Url> {
        match self {
            Self::String(value) => url::Url::parse(&value)
                .map_err(|e| URLParseError::new_err(format!("Cannot parse url: {:?}", e))),
            Self::UrllibParseResult(parse_result) => Ok(parse_result.0.clone()),
        }
    }
}
