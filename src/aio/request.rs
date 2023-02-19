use pyo3::prelude::*;

use crate::primitives;


#[pyclass]
pub struct AsyncRequest {
    pub method: primitives::HTTPMethod,
    pub url: primitives::URL,
    pub headers: Option<std::collections::HashMap<String, String>>
}

impl AsyncRequest {
    pub fn build(&self, client: &reqwest::Client) -> reqwest::Request {
        let mut request = client.request(
            self.method.0.clone(),
            self.url.0.clone()
        );
        if let Some(ref headers) = self.headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }

        request.build().unwrap()
    }
}


#[pymethods]
impl AsyncRequest {
    // TODO: More params
    #[new]
    pub fn new(
        method: primitives::HTTPMethod,
        url: primitives::URL,
        headers: Option<std::collections::HashMap<String, String>>
    ) -> Self {
        AsyncRequest {
            method, url, headers
        }
    }
}



// #[pyclass]
// pub struct AsyncRequestBuilder(pub std::cell::Cell<reqwest::RequestBuilder>);


// #[pymethods]
// impl AsyncRequestBuilder {
//     pub fn add_basic_auth<'rt>(
//         &mut self,
//         username: &str,
//         password: Option<&str>,
//     ) {

//         let p = self.0.sw;

//     }
// }


pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "request")?;
    submod.add_class::<AsyncRequest>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
