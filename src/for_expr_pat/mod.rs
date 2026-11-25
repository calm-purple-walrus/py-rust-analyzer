use crate::types::{AstForExpr, AstPat};
use pyo3::prelude::*;

#[pymethods]
impl AstForExpr {
    /// `fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
