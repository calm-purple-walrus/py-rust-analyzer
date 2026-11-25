use crate::types::{AstExternBlock, AstExternItemList};
use pyo3::prelude::*;

#[pymethods]
impl AstExternBlock {
    /// `fn extern_item_list(&self) -> Option<ExternItemList>`
    fn extern_item_list(&self) -> Option<AstExternItemList> {
        self.inner
            .extern_item_list()
            .map(|inner| AstExternItemList { inner })
    }
}
