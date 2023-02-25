use pyo3::prelude::*;

use crate::{
    aio::multipart::form::Multipart,
    exceptions::{wrap_reqwest_error, BorrowingError, RACE_CONDITION_ERROR_MSG},
    py2rs::{
        self,
        base::ToNative,
        duration::PyDurationAnalog,
        query::{QueryParam, QueryVecParam},
    },
};

#[pyclass]
pub struct Request {
    pub method: py2rs::http_method::HTTPMethod,
    pub url: py2rs::url::URL,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub query: Option<Vec<(String, String)>>,
    pub form: Option<std::collections::HashMap<String, String>>,
    pub bearer_auth: Option<String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<std::time::Duration>,
    pub multipart: Option<std::cell::RefCell<Option<reqwest::multipart::Form>>>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Request {
    pub fn build(&self, client: &reqwest::Client) -> PyResult<reqwest::Request> {
        let mut request = client.request(self.method.to_native()?, self.url.to_native()?);
        if let Some(ref headers) = self.headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(ref query) = self.query {
            request = request.query(query);
        }
        if let Some(ref form) = self.form {
            request = request.form(form);
        }
        if let Some(ref bearer_auth) = self.bearer_auth {
            request = request.bearer_auth(bearer_auth);
        }
        if let Some(ref body) = self.body {
            request = request.body(body.clone());
        }
        if let Some(ref timeout) = self.timeout {
            request = request.timeout(*timeout);
        }
        if let Some(ref username) = self.username {
            request = request.basic_auth(username.clone(), self.password.clone());
        }
        if let Some(ref multipart) = self.multipart {
            request = request.multipart(
                multipart
                    .borrow_mut()
                    .take()
                    .ok_or(BorrowingError::new_err(RACE_CONDITION_ERROR_MSG))?,
            );
        }

        Ok(request.build().map_err(wrap_reqwest_error)?)
    }
}

#[pymethods]
impl Request {
    // TODO: More params
    #[new]
    pub fn new(
        method: py2rs::http_method::HTTPMethod,
        url: py2rs::url::URL,
        headers: Option<std::collections::HashMap<String, String>>,
        query: Option<std::collections::HashMap<String, QueryParam>>,
        form: Option<std::collections::HashMap<String, String>>,
        bearer_auth: Option<String>,
        body: Option<Vec<u8>>,
        timeout: Option<PyDurationAnalog>,
        multipart: Option<&PyCell<Multipart>>,
        username: Option<String>,
        password: Option<String>,
    ) -> PyResult<Self> {
        Ok(Request {
            method,
            url,
            headers,
            query: {
                query.and_then(|query| {
                    let mut query_vec = vec![];
                    for (key, value) in query {
                        match value {
                            QueryParam::Array(array) => {
                                for elem in array {
                                    query_vec.push((
                                        key.clone(),
                                        match elem {
                                            QueryVecParam::Integer(value) => value.to_string(),
                                            QueryVecParam::Boolean(value) => value.to_string(),
                                            QueryVecParam::Number(value) => value.to_string(),
                                            QueryVecParam::Null(_) => "null".to_string(),
                                            QueryVecParam::String(value) => value.to_string(),
                                        },
                                    ));
                                }
                            }
                            QueryParam::Integer(value) => query_vec.push((key, value.to_string())),
                            QueryParam::Boolean(value) => query_vec.push((key, value.to_string())),
                            QueryParam::Number(value) => query_vec.push((key, value.to_string())),
                            QueryParam::Null(_) => query_vec.push((key, "null".to_string())),
                            QueryParam::String(value) => query_vec.push((key, value.to_string())),
                        };
                    }
                    Some(query_vec)
                })
            },
            form,
            bearer_auth,
            body,
            username,
            password,
            multipart: {
                multipart.and_then(|inner| {
                    Some(std::cell::RefCell::new(
                        inner.borrow_mut().0.borrow_mut().take(),
                    ))
                })
            },
            timeout: match timeout {
                Some(inner) => Some(inner.to_native()?),
                None => None,
            },
        })
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "request")?;
    submod.add_class::<Request>()?;
    library.add_class::<Request>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
