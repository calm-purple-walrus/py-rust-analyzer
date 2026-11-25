use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Field, Variant};

#[pymethods]
impl Field {
    /// `pub fn parent_def(&self, _db: &dyn HirDatabase) -> Variant`
    fn parent_def(&self, db: &Database) -> Variant {
        Variant { inner: attach_db(&db.db, || self.inner.parent_def(&db.db)) }
    }
}
