use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, InFileModuleSource};

#[pymethods]
impl Module {
    /// `pub fn definition_source(self, db: &dyn HirDatabase) -> InFile<ModuleSource>`
    fn definition_source(&self, db: &Database) -> InFileModuleSource {
        InFileModuleSource { inner: attach_db(&db.db, || self.inner.definition_source(&db.db)) }
    }
}
