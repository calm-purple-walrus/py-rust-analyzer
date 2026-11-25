use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::prelude::*;

use crate::types::Trait;

#[pymethods]
impl Trait {
    /// `impl Hash for Trait`
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        hasher.finish()
    }
}
