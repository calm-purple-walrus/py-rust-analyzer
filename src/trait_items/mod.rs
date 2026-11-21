use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{AssocItem, Trait};

#[pymethods]
impl Trait {
    /// `pub fn items(self, db: &dyn HirDatabase) -> Vec<AssocItem>`
    fn items(&self, db: &Database) -> Vec<AssocItem> {
        attach_db(&db.db, || self.inner.items(&db.db)).into_iter().map(|inner| AssocItem { inner }).collect()
    }
}
