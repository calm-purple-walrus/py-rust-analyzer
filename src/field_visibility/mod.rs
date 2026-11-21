use pyo3::prelude::*;
use ra_ap_hir::{HasVisibility, attach_db};

use crate::database::Database;
use crate::types::{Field, Visibility};

#[pymethods]
impl Field {
    /// `impl HasVisibility for Field`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}
