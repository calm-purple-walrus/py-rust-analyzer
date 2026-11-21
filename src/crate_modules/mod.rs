use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, Module};

#[pymethods]
impl Crate {
    /// `pub fn modules(self, db: &dyn HirDatabase) -> Vec<Module>`
    fn modules(&self, db: &Database) -> Vec<Module> {
        attach_db(&db.db, || self.inner.modules(&db.db)).into_iter().map(|inner| Module { inner }).collect()
    }
}
