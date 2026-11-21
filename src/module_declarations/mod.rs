use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, ModuleDef};

#[pymethods]
impl Module {
    /// `pub fn declarations(self, db: &dyn HirDatabase) -> Vec<ModuleDef>`
    fn declarations(&self, db: &Database) -> Vec<ModuleDef> {
        attach_db(&db.db, || self.inner.declarations(&db.db)).into_iter().map(|inner| ModuleDef { inner }).collect()
    }
}
