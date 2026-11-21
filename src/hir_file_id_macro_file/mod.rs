use pyo3::prelude::*;

use crate::types::{HirFileId, MacroCallId};

#[pymethods]
impl HirFileId {
    /// The variant `HirFileId::MacroFile(MacroCallId)`, read: its `MacroCallId` if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_MacroFile")]
    fn as_macro_file(&self) -> Option<MacroCallId> {
        match self.inner {
            ra_ap_hir::HirFileId::MacroFile(inner) => Some(MacroCallId { inner }),
            _ => None,
        }
    }
}
