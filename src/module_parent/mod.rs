use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module};

#[pymethods]
impl Module {
    /// `pub fn parent(self, db: &dyn HirDatabase) -> Option<Module>`
    fn parent(&self, db: &Database) -> Option<Module> {
        attach_db(&db.db, || self.inner.parent(&db.db)).map(|inner| Module { inner })
    }
}
