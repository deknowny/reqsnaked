use pyo3::prelude::*;

use crate::exceptions::MIMEParseError;

#[pyclass(module = "reqsnaked")]
pub struct Part {
    pub name: String,
    pub inner: std::cell::RefCell<Option<reqwest::multipart::Part>>,
}

#[derive(Debug, FromPyObject)]
pub enum PartData {
    Text(String),
    Bytes(Vec<u8>),
}

#[pymethods]
impl Part {
    #[new]
    pub fn new(
        name: String,
        value: PartData,
        filename: Option<String>,
        mime: Option<&str>,
    ) -> PyResult<Part> {
        // TODO
        let mut inner = match value {
            PartData::Text(content) => reqwest::multipart::Part::text(content),
            PartData::Bytes(content) => reqwest::multipart::Part::bytes(content),
        };
        if let Some(filename) = filename {
            inner = inner.file_name(filename);
        }
        if let Some(mime) = mime {
            inner = inner
                .mime_str(mime)
                .map_err(|e| MIMEParseError::new_err(format!("Cannot parse MIME type: {:?}", e)))?;
        }
        Ok(Part {
            name,
            inner: std::cell::RefCell::new(Some(inner)),
        })
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "part")?;
    submod.add_class::<Part>()?;
    library.add_class::<Part>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
