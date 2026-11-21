use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, DisplayTarget};

#[pymethods]
impl Crate {
    /// `pub fn to_display_target(self, db: &dyn HirDatabase) -> DisplayTarget`
    fn to_display_target(&self, db: &Database) -> DisplayTarget {
        DisplayTarget { inner: attach_db(&db.db, || self.inner.to_display_target(&db.db)) }
    }
}
