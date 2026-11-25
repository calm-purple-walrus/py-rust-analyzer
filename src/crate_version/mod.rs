use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::Crate;

#[pymethods]
impl Crate {
    /// `pub fn version(self, db: &dyn HirDatabase) -> Option<String>`
    fn version(&self, db: &Database) -> Option<String> {
        attach_db(&db.db, || self.inner.version(&db.db))
    }
}
