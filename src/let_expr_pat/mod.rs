use pyo3::prelude::*;

use crate::types::{AstLetExpr, AstPat};

#[pymethods]
impl AstLetExpr {
    /// `pub fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
