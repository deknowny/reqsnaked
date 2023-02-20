use pyo3::prelude::*;
use pyo3::types::PyDelta;

use crate::py2rs::base::ToNative;

#[derive(FromPyObject)]
pub enum PyDurationAnalog<'a> {
    TimeDelta(&'a PyDelta),
    Seconds(u64),
}

impl ToNative for PyDurationAnalog<'_> {
    type Native = PyResult<std::time::Duration>;

    fn to_native(&self) -> Self::Native {
        match self {
            Self::TimeDelta(value) => {
                let total_seconds: u64 = value.call_method0("total_seconds")?.extract()?;
                Ok(std::time::Duration::from_secs(total_seconds))
            }
            Self::Seconds(value) => Ok(std::time::Duration::from_secs(*value)),
        }
    }
}
