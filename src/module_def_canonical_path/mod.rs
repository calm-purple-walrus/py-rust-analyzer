use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Edition, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// `pub fn canonical_path(&self, db: &dyn HirDatabase, edition: Edition) -> Option<String>`
    fn canonical_path(&self, db: &Database, edition: &Edition) -> Option<String> {
        attach_db(&db.db, || self.inner.canonical_path(&db.db, edition.inner))
    }
}
