use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// `pub fn module(self, db: &dyn HirDatabase) -> Option<Module>`
    fn module(&self, db: &Database) -> Option<Module> {
        attach_db(&db.db, || self.inner.module(&db.db)).map(|inner| Module { inner })
    }
}
