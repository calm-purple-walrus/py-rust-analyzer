use pyo3::prelude::*;

use crate::types::{AssocItem, Function};

#[pymethods]
impl AssocItem {
    /// The variant `AssocItem::Function(Function)`, read: its `Function` if the value is that variant, otherwise
    /// `None`.
    #[pyo3(name = "as_Function")]
    fn as_function(&self) -> Option<Function> {
        match self.inner {
            ra_ap_hir::AssocItem::Function(inner) => Some(Function { inner }),
            _ => None,
        }
    }
}
