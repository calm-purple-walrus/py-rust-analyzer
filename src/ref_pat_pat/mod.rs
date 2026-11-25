use crate::types::{AstPat, AstRefPat};
use pyo3::prelude::*;

#[pymethods]
impl AstRefPat {
    /// `fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
