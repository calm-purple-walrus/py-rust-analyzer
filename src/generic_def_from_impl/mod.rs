use crate::types::{GenericDef, Impl};
use pyo3::prelude::*;

#[pymethods]
impl GenericDef {
    /// `impl From<Impl> for GenericDef`
    #[staticmethod]
    #[pyo3(name = "from_Impl")]
    fn from_impl(impl_: &Impl) -> GenericDef {
        GenericDef {
            inner: impl_.inner.into(),
        }
    }
}
