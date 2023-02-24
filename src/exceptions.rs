use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError},
};

pub const RACE_CONDITION_ERROR_MSG: &'static str = r#"Due Rust's memory managment approach of owning,
you cannot use some instances for some kind of
stuff twice as they are gone.

There are 2 cases you've got this error:
1) You passed a non-clonable instance to another that requires owning
2) You tried using method with owning twice (i.e. reading response's body twice)

Potential solutions:
1) Do not share instances, create new on every time you use it
2) Do not do this. Try another way to solve your problem
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
