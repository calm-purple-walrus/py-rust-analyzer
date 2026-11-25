use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::Module;

#[pymethods]
impl Module {
    /// `pub fn children(self, db: &dyn HirDatabase) -> impl Iterator<Item = Module>`, as a list.
    fn children(&self, db: &Database) -> Vec<Module> {
        attach_db(&db.db, || self.inner.children(&db.db).map(|inner| Module { inner }).collect())
    }
}
