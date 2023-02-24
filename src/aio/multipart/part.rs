use pyo3::{prelude::*, types::PyType};

#[pyclass]
pub struct Part(
    pub std::cell::RefCell<Option<reqwest::multipart::Part>>
);


#[pymethods]
impl Part {
    #[classmethod]
    pub fn text(
        _cls: &PyType,
        value: String
    ) -> Part {
        // TODO
        let new_part = reqwest::multipart::Part::text(value);
        Part(
            std::cell::RefCell::new(Some(new_part))
        )
    }

    #[classmethod]
    pub fn bytes(
        _cls: &PyType,
        value: Vec<u8>
    ) -> Part {
        // TODO
        let new_part = reqwest::multipart::Part::bytes(value);
        Part(
            std::cell::RefCell::new(Some(new_part))
        )
    }

    pub fn set_filename<'rt>(
        slf: PyRefMut<'rt, Self>,
        filename: String,
    ) -> PyRefMut<'rt, Self> {
        // TODO
        let part = slf.0.borrow_mut().take().unwrap();
        let new_part = part.file_name(filename);
        slf.0.replace(Some(new_part));
        slf
    }

    pub fn set_mime<'rt>(
        slf: PyRefMut<'rt, Self>,
        mime: &str,
    ) -> PyResult<PyRefMut<'rt, Self>> {
        // TODO
        let part = slf.0.borrow_mut().take().unwrap();
        let new_part = part.mime_str(mime).unwrap();
        slf.0.replace(Some(new_part));
        Ok(slf)
    }

}


pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "part")?;
    submod.add_class::<Part>()?;
    library.add_class::<Part>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
