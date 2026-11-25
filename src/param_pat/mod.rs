use crate::types::{AstParam, AstPat};
use pyo3::prelude::*;

#[pymethods]
impl AstParam {
    /// `fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
