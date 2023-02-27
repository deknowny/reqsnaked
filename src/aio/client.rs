use pyo3::prelude::*;

use crate::aio::request::Request;
use crate::aio::response::AsyncResponse;
use crate::exceptions::wrap_reqwest_error;

#[pyclass]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn py_awaitable_request<'rt>(
        &self,
        client: reqwest::Client,
        request: reqwest::Request,
        py: Python<'rt>,
    ) -> PyResult<&'rt PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let response = client.execute(request).await.map_err(wrap_reqwest_error)?;
            Ok(AsyncResponse {
                status: response.status(),
                version: response.version(),
                headers: response.headers().clone(),
                response: std::cell::RefCell::new(Some(response)),
            })
        })
    }
}

#[pymethods]
impl Client {
    #[new]
    pub fn new(
        user_agent: Option<String>,
        headers: Option<std::collections::HashMap<String, String>>,
        store_cookie: Option<bool>,
        max_allowed_redirects: Option<usize>,
        danger_accept_invalid_certs: Option<bool>
    ) -> PyResult<Self> {
        let mut client = reqwest::Client::builder();
        client = client.use_rustls_tls();
        if let Some(ref user_agent) = user_agent {
            client = client.user_agent(user_agent);
        }
        let mut default_headers_map = reqwest::header::HeaderMap::new();
        if let Some(default_headers) = headers {
            for (key, value) in default_headers {
                // TODO: headers wrapper
                default_headers_map.insert(
                    reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    reqwest::header::HeaderValue::from_str(&value).unwrap(),
                );
            }
            client = client.default_headers(default_headers_map);
        }
        if let Some(store_cookie) = store_cookie {
            client = client.cookie_store(store_cookie);
        }
        if let Some(max_allowed_redirects) = max_allowed_redirects {
            client = client.redirect(reqwest::redirect::Policy::limited(max_allowed_redirects))
        }
        if let Some(danger_accept_invalid_certs) = danger_accept_invalid_certs {
            client = client.danger_accept_invalid_certs(danger_accept_invalid_certs);
        }

        match client.build() {
            Err(err) => Err(wrap_reqwest_error(err)),
            Ok(client) => Ok(Client { client }),
        }
    }

    pub fn send<'rt>(&self, request: &PyCell<Request>, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        let client = self.client.clone();
        let request = request.borrow().build(&client)?;
        self.py_awaitable_request(client, request, py)
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "client")?;
    submod.add_class::<Client>()?;
    library.add_class::<Client>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
