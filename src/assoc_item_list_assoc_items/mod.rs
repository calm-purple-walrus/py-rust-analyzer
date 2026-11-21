use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstAssocItem, AstAssocItemList};

#[pymethods]
impl AstAssocItemList {
    /// `pub fn assoc_items(&self) -> AstChildren<AssocItem>`
    fn assoc_items(&self) -> Iter {
        Iter::new(self.inner.assoc_items(), |py, inner| Ok(Py::new(py, AstAssocItem { inner })?.into_any()), None)
    }
}
