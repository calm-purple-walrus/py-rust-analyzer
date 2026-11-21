use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::prelude::*;

use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `impl Hash for SyntaxNode`
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (*self.inner).hash(&mut hasher);
        hasher.finish()
    }
}
