use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, Name};

#[pymethods]
impl Module {
    /// `pub fn name(self, db: &dyn HirDatabase) -> Option<Name>`
    fn name(&self, db: &Database) -> Option<Name> {
        attach_db(&db.db, || self.inner.name(&db.db)).map(|inner| Name { inner })
    }
}
