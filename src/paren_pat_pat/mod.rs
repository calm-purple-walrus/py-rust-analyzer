use crate::types::{AstParenPat, AstPat};
use pyo3::prelude::*;

#[pymethods]
impl AstParenPat {
    /// `fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
