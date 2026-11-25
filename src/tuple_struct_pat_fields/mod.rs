use crate::iter::Iter;
use crate::types::{AstPat, AstTupleStructPat};
use pyo3::prelude::*;

#[pymethods]
impl AstTupleStructPat {
    /// `fn fields(&self) -> AstChildren<Pat>`
    fn fields(&self) -> Iter {
        Iter::new(
            self.inner.fields(),
            |py, inner| Ok(Py::new(py, AstPat { inner })?.into_any()),
            None,
        )
    }
}
