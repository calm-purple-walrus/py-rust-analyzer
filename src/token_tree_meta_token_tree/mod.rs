use pyo3::prelude::*;

use crate::types::{AstTokenTreeMeta, AstTokenTree};

#[pymethods]
impl AstTokenTreeMeta {
    /// `pub fn token_tree(&self) -> Option<TokenTree>`
    fn token_tree(&self) -> Option<AstTokenTree> {
        self.inner.token_tree().map(|inner| AstTokenTree { inner })
    }
}
