use crate::types::{Function, GenericDef};
use pyo3::prelude::*;

#[pymethods]
impl GenericDef {
    /// `impl From<Function> for GenericDef`
    #[staticmethod]
    #[pyo3(name = "from_Function")]
    fn from_function(function: &Function) -> GenericDef {
        GenericDef {
            inner: function.inner.into(),
        }
    }
}
