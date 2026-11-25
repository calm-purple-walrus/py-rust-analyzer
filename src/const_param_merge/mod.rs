use crate::types::{ConstParam, TypeOrConstParam};
use pyo3::prelude::*;

#[pymethods]
impl ConstParam {
    /// `fn merge(self) -> TypeOrConstParam`
    fn merge(&self) -> TypeOrConstParam {
        TypeOrConstParam {
            inner: self.inner.merge(),
        }
    }
}
