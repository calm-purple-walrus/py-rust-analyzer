use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::Module;

#[pymethods]
impl Module {
    /// `pub fn is_inline(self, db: &dyn HirDatabase) -> bool`
    fn is_inline(&self, db: &Database) -> bool {
        attach_db(&db.db, || self.inner.is_inline(&db.db))
    }
}
