use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, Crate};

#[pymethods]
impl Module {
    /// `pub fn krate(self, db: &dyn HirDatabase) -> Crate`
    fn krate(&self, db: &Database) -> Crate {
        Crate { inner: attach_db(&db.db, || self.inner.krate(&db.db)) }
    }
}
