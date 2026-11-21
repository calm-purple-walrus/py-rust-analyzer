use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Struct, StructKind};

#[pymethods]
impl Struct {
    /// `pub fn kind(self, db: &dyn HirDatabase) -> StructKind`
    fn kind(&self, db: &Database) -> StructKind {
        StructKind { inner: attach_db(&db.db, || self.inner.kind(&db.db)) }
    }
}
