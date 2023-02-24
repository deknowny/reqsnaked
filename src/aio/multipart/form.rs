use pyo3::prelude::*;

use crate::exceptions::{BorrowingError, RACE_CONDITION_ERROR_MSG};

#[pyclass]
pub struct Multipart(pub std::cell::RefCell<Option<reqwest::multipart::Form>>);

#[pymethods]
impl Multipart {
    #[new]
    #[args(parts = "*")]
    pub fn new(parts: Vec<&PyCell<crate::aio::multipart::part::Part>>) -> PyResult<Multipart> {
        let mut new_form = reqwest::multipart::Form::new();
        for part in parts {
            new_form = new_form.part(
                part.borrow().name.clone(),
                part.borrow()
                    .inner
                    .borrow_mut()
                    .take()
                    .ok_or(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG))?,
            );
        }
        Ok(Multipart(std::cell::RefCell::new(Some(new_form))))
    }

    pub fn boundary(&self) -> PyResult<String> {
        Ok(self
            .0
            .borrow()
            .as_ref()
            .ok_or(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG))?
            .boundary()
            .to_string())
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "form")?;
    submod.add_class::<Multipart>()?;
    library.add_class::<Multipart>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
