use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Adt, Module};

#[pymethods]
impl Adt {
    /// `pub fn module(self, db: &dyn HirDatabase) -> Module`
    fn module(&self, db: &Database) -> Module {
        Module { inner: attach_db(&db.db, || self.inner.module(&db.db)) }
    }
}
