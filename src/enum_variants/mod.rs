use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Enum, EnumVariant};

#[pymethods]
impl Enum {
    /// `pub fn variants(self, db: &dyn HirDatabase) -> Vec<EnumVariant>`
    fn variants(&self, db: &Database) -> Vec<EnumVariant> {
        attach_db(&db.db, || self.inner.variants(&db.db)).into_iter().map(|inner| EnumVariant { inner }).collect()
    }
}
