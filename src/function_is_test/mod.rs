use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::Function;

#[pymethods]
impl Function {
    /// `pub fn is_test(self, db: &dyn HirDatabase) -> bool`
    fn is_test(&self, db: &Database) -> bool {
        attach_db(&db.db, || self.inner.is_test(&db.db))
    }
}
