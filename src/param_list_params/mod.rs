use crate::iter::Iter;
use crate::types::{AstParam, AstParamList};
use pyo3::prelude::*;

#[pymethods]
impl AstParamList {
    /// `fn params(&self) -> AstChildren<Param>`
    fn params(&self) -> Iter {
        Iter::new(
            self.inner.params(),
            |py, inner| Ok(Py::new(py, AstParam { inner })?.into_any()),
            None,
        )
    }
}
