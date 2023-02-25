use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError},
    prelude::*,
};

pub const RACE_CONDITION_ERROR_MSG: &'static str = r#"Due Rust's memory managment approach of borrowing,
you cannot use some instances for some kind of
stuff twice as they are gone.

There are 3 cases you've got this error:
1) You passed a non-clonable instance to another that requires owning
2) You tried using method with owning twice (i.e. reading response's body twice)
3) You tried using referencing after borrowing

Potential solutions:
1) Do not share instances, create new on every time you use it
2) Do not do this. Try another way to solve your problem
3) Swap calls order (referencing first)
inner types "#;

create_exception!(exceptions, BorrowingError, PyRuntimeError);

create_exception!(exceptions, BaseReqwestError, PyException);
create_exception!(exceptions, BodyError, BaseReqwestError);
create_exception!(exceptions, BuilderError, BaseReqwestError);
create_exception!(exceptions, ConnectionError, BaseReqwestError);
create_exception!(exceptions, DecodingError, BaseReqwestError);
create_exception!(exceptions, RedirectError, BaseReqwestError);
create_exception!(exceptions, TimeoutError, BaseReqwestError);
create_exception!(exceptions, StatusError, BaseReqwestError);
create_exception!(exceptions, RequestError, BaseReqwestError);
create_exception!(exceptions, UnknownError, BaseReqwestError);

create_exception!(exceptions, HTTPMethodParseError, PyException);
create_exception!(exceptions, URLParseError, PyException);
create_exception!(exceptions, MIMEParseError, PyException);

pub fn wrap_reqwest_error(error: reqwest::Error) -> pyo3::PyErr {
    if error.is_body() {
        return BodyError::new_err(format!("Body related error: {:?}", error));
    } else if error.is_connect() {
        return ConnectionError::new_err(format!("Could not connect to host: {:?}", error));
    } else if error.is_decode() {
        return DecodingError::new_err(format!("Response body decoding error: {:?}", error));
    } else if error.is_redirect() {
        return RedirectError::new_err(format!("Maximum redirect count was reached: {:?}", error));
    } else if error.is_timeout() {
        return TimeoutError::new_err(format!("Timeout has been reached: {:?}", error));
    } else if error.is_status() {
        return StatusError::new_err(format!("Error status code in the response: {:?}", error));
    } else if error.is_request() {
        return RequestError::new_err(format!("Request error: {:?}", error));
    } else if error.is_builder() {
        return BuilderError::new_err(format!("Could not build the request: {:?}", error));
    } else {
        return UnknownError::new_err(format!("Unknown error occured: {:?}", error));
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "exceptions")?;
    submod.add(stringify!(BorrowingError), py.get_type::<BorrowingError>())?;
    library.add(stringify!(BorrowingError), py.get_type::<BorrowingError>())?;

    submod.add(
        stringify!(BaseReqwestError),
        py.get_type::<BaseReqwestError>(),
    )?;
    library.add(
        stringify!(BaseReqwestError),
        py.get_type::<BaseReqwestError>(),
    )?;

    submod.add(stringify!(BodyError), py.get_type::<BodyError>())?;
    library.add(stringify!(BodyError), py.get_type::<BodyError>())?;

    submod.add(stringify!(BuilderError), py.get_type::<BuilderError>())?;
    library.add(stringify!(BuilderError), py.get_type::<BuilderError>())?;

    submod.add(
        stringify!(ConnectionError),
        py.get_type::<ConnectionError>(),
    )?;
    library.add(
        stringify!(ConnectionError),
        py.get_type::<ConnectionError>(),
    )?;

    submod.add(stringify!(DecodingError), py.get_type::<DecodingError>())?;
    library.add(stringify!(DecodingError), py.get_type::<DecodingError>())?;

    submod.add(stringify!(RedirectError), py.get_type::<RedirectError>())?;
    library.add(stringify!(RedirectError), py.get_type::<RedirectError>())?;

    submod.add(stringify!(TimeoutError), py.get_type::<TimeoutError>())?;
    library.add(stringify!(TimeoutError), py.get_type::<TimeoutError>())?;

    submod.add(stringify!(StatusError), py.get_type::<StatusError>())?;
    library.add(stringify!(StatusError), py.get_type::<StatusError>())?;

    submod.add(stringify!(RequestError), py.get_type::<RequestError>())?;
    library.add(stringify!(RequestError), py.get_type::<RequestError>())?;

    submod.add(stringify!(UnknownError), py.get_type::<UnknownError>())?;
    library.add(stringify!(UnknownError), py.get_type::<UnknownError>())?;

    submod.add(
        stringify!(HTTPMethodParseError),
        py.get_type::<HTTPMethodParseError>(),
    )?;
    library.add(
        stringify!(HTTPMethodParseError),
        py.get_type::<HTTPMethodParseError>(),
    )?;

    submod.add(stringify!(URLParseError), py.get_type::<URLParseError>())?;
    library.add(stringify!(URLParseError), py.get_type::<URLParseError>())?;

    submod.add(stringify!(MIMEParseError), py.get_type::<MIMEParseError>())?;
    library.add(stringify!(MIMEParseError), py.get_type::<MIMEParseError>())?;

    parent_module.add_submodule(submod)?;
    Ok(())
}
