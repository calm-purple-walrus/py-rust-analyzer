use pyo3::prelude::*;

use crate::types::{BuiltinType, Name};

#[pymethods]
impl BuiltinType {
    /// `pub fn name(self) -> Name`
    fn name(&self) -> Name {
        Name { inner: self.inner.name() }
    }
}
