use crate::types::{AstBlockExpr, AstBlockModifier};
use pyo3::prelude::*;

#[pymethods]
impl AstBlockExpr {
    /// `fn modifier(&self) -> Option<BlockModifier>`
    fn modifier(&self) -> Option<AstBlockModifier> {
        self.inner
            .modifier()
            .map(|inner| AstBlockModifier { inner })
    }
}
