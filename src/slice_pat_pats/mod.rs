use crate::iter::Iter;
use crate::types::{AstPat, AstSlicePat};
use pyo3::prelude::*;

#[pymethods]
impl AstSlicePat {
    /// `fn pats(&self) -> AstChildren<Pat>`
    fn pats(&self) -> Iter {
        Iter::new(
            self.inner.pats(),
            |py, inner| Ok(Py::new(py, AstPat { inner })?.into_any()),
            None,
        )
    }
}
