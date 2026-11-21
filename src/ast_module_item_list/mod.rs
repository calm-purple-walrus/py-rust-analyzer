use pyo3::prelude::*;

use crate::types::{AstModule, AstItemList};

#[pymethods]
impl AstModule {
    /// `pub fn item_list(&self) -> Option<ItemList>`
    fn item_list(&self) -> Option<AstItemList> {
        self.inner.item_list().map(|inner| AstItemList { inner })
    }
}
