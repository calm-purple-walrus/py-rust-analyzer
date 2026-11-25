use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, Module};

#[pymethods]
impl Crate {
    /// `pub fn root_module(self, db: &dyn HirDatabase) -> Module`
    fn root_module(&self, db: &Database) -> Module {
        Module { inner: attach_db(&db.db, || self.inner.root_module(&db.db)) }
    }
}
