use pyo3::prelude::*;

use crate::types::{Module, Visibility};

#[pymethods]
impl Visibility {
    /// The variant `Visibility::Module(ModuleId, VisibilityExplicitness)`, read: its module, as the `hir::Module`
    /// that `From<ModuleId>` makes of it, if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Module")]
    fn as_module(&self) -> Option<Module> {
        match self.inner {
            ra_ap_hir::Visibility::Module(module, _) => Some(Module { inner: module.into() }),
            _ => None,
        }
    }
}
