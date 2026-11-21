use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{EnumVariant, Field};

#[pymethods]
impl EnumVariant {
    /// `pub fn fields(self, db: &dyn HirDatabase) -> Vec<Field>`
    fn fields(&self, db: &Database) -> Vec<Field> {
        attach_db(&db.db, || self.inner.fields(&db.db)).into_iter().map(|inner| Field { inner }).collect()
    }
}
