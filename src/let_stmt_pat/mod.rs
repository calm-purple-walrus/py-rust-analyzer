use pyo3::prelude::*;

use crate::types::{AstLetStmt, AstPat};

#[pymethods]
impl AstLetStmt {
    /// `pub fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
