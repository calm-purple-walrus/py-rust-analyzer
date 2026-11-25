use crate::iter::Iter;
use crate::types::{AstExternItem, AstExternItemList};
use pyo3::prelude::*;

#[pymethods]
impl AstExternItemList {
    /// `fn extern_items(&self) -> AstChildren<ExternItem>`
    fn extern_items(&self) -> Iter {
        Iter::new(
            self.inner.extern_items(),
            |py, inner| Ok(Py::new(py, AstExternItem { inner })?.into_any()),
            None,
        )
    }
}
