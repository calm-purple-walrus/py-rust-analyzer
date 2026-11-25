use crate::types::{GenericDef, TypeAlias};
use pyo3::prelude::*;

#[pymethods]
impl GenericDef {
    /// `impl From<TypeAlias> for GenericDef`
    #[staticmethod]
    #[pyo3(name = "from_TypeAlias")]
    fn from_type_alias(type_alias: &TypeAlias) -> GenericDef {
        GenericDef {
            inner: type_alias.inner.into(),
        }
    }
}
