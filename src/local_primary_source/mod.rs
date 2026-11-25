use crate::database::Database;
use crate::types::{Local, LocalSource};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl Local {
    /// `fn primary_source(self, db: &dyn HirDatabase) -> LocalSource<'db>`
    fn primary_source(&self, py: Python<'_>, db: &Database) -> LocalSource {
        let inner = attach_db(&db.db, || self.inner.primary_source(&db.db));
        LocalSource {
            inner,
            _db: self.db.clone_ref(py),
        }
    }
}
