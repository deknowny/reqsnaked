use pyo3::prelude::*;

#[pyclass]
pub struct AsyncMultipart(
    pub std::cell::RefCell<Option<reqwest::multipart::Form>>
);

#[pymethods]
impl AsyncMultipart {
    #[new]
    pub fn new(
    ) -> AsyncMultipart {
        AsyncMultipart(
            std::cell::RefCell::new(
                Some(reqwest::multipart::Form::new())
            )
        )
    }

    pub fn text<'rt>(
        slf: PyRefMut<'rt, Self>,
        name: String,
        value: String
    ) -> PyRefMut<'rt, Self> {
        // TODO
        let form = slf.0.borrow_mut().take().unwrap();
        let new_form = form.text(name, value);
        slf.0.replace(Some(new_form));
        slf
    }

    pub fn part<'rt>(
        slf: PyRefMut<'rt, Self>,
        name: String,
        part: &PyCell<crate::aio::multipart::part::Part>,
    ) -> PyRefMut<'rt, Self> {
        // TODO
        let part = part.borrow_mut().0.borrow_mut().take().unwrap();
        let form = slf.0.borrow_mut().take().unwrap();
        let new_form = form.part(name, part);
        slf.0.replace(Some(new_form));
        slf
    }

    pub fn boundary(&self) -> String {
        self.0.borrow().as_ref().unwrap().boundary().to_string()
    }
}


pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "form")?;
    submod.add_class::<AsyncMultipart>()?;
    library.add_class::<AsyncMultipart>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
