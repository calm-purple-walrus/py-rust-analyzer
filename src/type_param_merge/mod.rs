use crate::types::{TypeOrConstParam, TypeParam};
use pyo3::prelude::*;

#[pymethods]
impl TypeParam {
    /// `fn merge(self) -> TypeOrConstParam`
    fn merge(&self) -> TypeOrConstParam {
        TypeOrConstParam {
            inner: self.inner.merge(),
        }
    }
}
