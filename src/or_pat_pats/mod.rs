use crate::iter::Iter;
use crate::types::{AstOrPat, AstPat};
use pyo3::prelude::*;

#[pymethods]
impl AstOrPat {
    /// `fn pats(&self) -> AstChildren<Pat>`
    fn pats(&self) -> Iter {
        Iter::new(
            self.inner.pats(),
            |py, inner| Ok(Py::new(py, AstPat { inner })?.into_any()),
            None,
        )
    }
}
