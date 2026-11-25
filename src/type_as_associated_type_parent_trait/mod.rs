use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Trait, Type};

#[pymethods]
impl Type {
    /// `pub fn as_associated_type_parent_trait(&self, db: &'db dyn HirDatabase) -> Option<Trait>`
    fn as_associated_type_parent_trait(&self, db: &Database) -> Option<Trait> {
        attach_db(&db.db, || self.inner.as_associated_type_parent_trait(&db.db)).map(|inner| Trait { inner })
    }
}
