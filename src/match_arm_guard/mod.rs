use crate::types::{AstMatchArm, AstMatchGuard};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchArm {
    /// `fn guard(&self) -> Option<MatchGuard>`
    fn guard(&self) -> Option<AstMatchGuard> {
        self.inner.guard().map(|inner| AstMatchGuard { inner })
    }
}
