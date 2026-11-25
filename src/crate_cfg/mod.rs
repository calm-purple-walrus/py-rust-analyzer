use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{CfgOptions, Crate};

#[pymethods]
impl Crate {
    /// `pub fn cfg<'db>(&self, db: &'db dyn HirDatabase) -> &'db CfgOptions`. Copied, because `apply_diff` changes
    /// a `CfgOptions`.
    fn cfg(&self, db: &Database) -> CfgOptions {
        CfgOptions { inner: attach_db(&db.db, || self.inner.cfg(&db.db).clone()) }
    }
}
