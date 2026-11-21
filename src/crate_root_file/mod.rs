use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, FileId};

#[pymethods]
impl Crate {
    /// `pub fn root_file(self, db: &dyn HirDatabase) -> FileId`
    fn root_file(&self, db: &Database) -> FileId {
        FileId { inner: attach_db(&db.db, || self.inner.root_file(&db.db)) }
    }
}
