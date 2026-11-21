use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Name, Trait};

#[pymethods]
impl Trait {
    /// `pub fn name(self, db: &dyn HirDatabase) -> Name`
    fn name(&self, db: &Database) -> Name {
        Name { inner: attach_db(&db.db, || self.inner.name(&db.db)) }
    }
}
