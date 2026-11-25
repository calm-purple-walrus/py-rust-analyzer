use pyo3::prelude::*;

use crate::types::{AstAssocItemList, AstImpl};

#[pymethods]
impl AstImpl {
    /// `pub fn assoc_item_list(&self) -> Option<AssocItemList>`
    fn assoc_item_list(&self) -> Option<AstAssocItemList> {
        self.inner.assoc_item_list().map(|inner| AstAssocItemList { inner })
    }
}
