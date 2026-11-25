use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{GenericDef, GenericParam};

#[pymethods]
impl GenericDef {
    /// `pub fn params(self, db: &dyn HirDatabase) -> Vec<GenericParam>`
    fn params(&self, db: &Database) -> Vec<GenericParam> {
        attach_db(&db.db, || self.inner.params(&db.db)).into_iter().map(|inner| GenericParam { inner }).collect()
    }
}
