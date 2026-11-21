use pyo3::prelude::*;
use ra_ap_syntax::ast::HasModuleItem;

use crate::iter::Iter;
use crate::types::{AstItemList, Item, SourceFile};

#[pymethods]
impl SourceFile {
    /// `impl HasModuleItem for SourceFile`: `fn items(&self) -> AstChildren<ast::Item>`. Python takes
    /// each item with `impl<N: AstNode> Iterator for AstChildren<N>`: `fn next(&mut self) -> Option<N>`.
    fn items(&self) -> Iter {
        Iter::new(self.inner.items(), |py, inner| Ok(Py::new(py, Item { inner })?.into_any()), None)
    }
}

#[pymethods]
impl AstItemList {
    /// `impl HasModuleItem for ItemList`: `fn items(&self) -> AstChildren<ast::Item>`
    fn items(&self) -> Iter {
        Iter::new(self.inner.items(), |py, inner| Ok(Py::new(py, Item { inner })?.into_any()), None)
    }
}
