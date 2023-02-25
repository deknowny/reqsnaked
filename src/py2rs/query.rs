use pyo3::prelude::*;
use serde::Serialize;

#[derive(Debug, FromPyObject, Serialize)]
pub enum QueryVecParam {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Null(Option<isize>),
}

#[derive(Debug, FromPyObject, Serialize)]
pub enum QueryParam {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Null(Option<isize>),
    Array(Vec<QueryVecParam>),
}
