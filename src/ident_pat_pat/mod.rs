use pyo3::prelude::*;

use crate::types::{AstIdentPat, AstPat};

#[pymethods]
impl AstIdentPat {
    /// `pub fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
