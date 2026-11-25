use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::prelude::*;

use crate::types::Adt;

#[pymethods]
impl Adt {
    /// `impl Hash for Adt`
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        hasher.finish()
    }
}
