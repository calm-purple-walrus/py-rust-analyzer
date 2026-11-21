use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Impl, Trait};

#[pymethods]
impl Impl {
    /// `pub fn trait_(self, db: &dyn HirDatabase) -> Option<Trait>`
    fn trait_(&self, db: &Database) -> Option<Trait> {
        attach_db(&db.db, || self.inner.trait_(&db.db)).map(|inner| Trait { inner })
    }
}
