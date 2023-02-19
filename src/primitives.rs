use std::str::FromStr;

use pyo3::{prelude::*, types::PyType};

#[derive(Clone)]
#[pyclass]
pub struct HTTPMethod(pub reqwest::Method);


#[pymethods]
impl HTTPMethod {
    #[classmethod]
    pub fn from_str(
        _cls: &PyType,
        name: &str,
    ) -> PyResult<Self> {
        Ok(
            HTTPMethod(
                reqwest::Method::from_str(name).unwrap()  // TODO
            )
        )
    }
}

#[derive(Clone)]
#[pyclass]
pub struct URL(pub url::Url);


#[pymethods]
impl URL {
    #[classmethod]
    pub fn from_str(
        _cls: &PyType,
        name: &str,
    ) -> PyResult<Self> {
        Ok(
            URL(
                name.parse::<url::Url>().unwrap()
            ) // TODO
        )
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



pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "primitives")?;
    submod.add_class::<HTTPMethod>()?;
    submod.add_class::<URL>()?;
    submod.add_class::<StatusCode>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
