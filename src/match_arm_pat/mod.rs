use crate::types::{AstMatchArm, AstPat};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchArm {
    /// `fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
