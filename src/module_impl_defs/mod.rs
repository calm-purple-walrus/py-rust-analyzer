use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, Impl};

#[pymethods]
impl Module {
    /// `pub fn impl_defs(self, db: &dyn HirDatabase) -> Vec<Impl>`
    fn impl_defs(&self, db: &Database) -> Vec<Impl> {
        attach_db(&db.db, || self.inner.impl_defs(&db.db)).into_iter().map(|inner| Impl { inner }).collect()
    }
}
