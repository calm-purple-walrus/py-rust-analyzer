use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, Edition};

#[pymethods]
impl Crate {
    /// `pub fn edition(self, db: &dyn HirDatabase) -> Edition`
    fn edition(&self, db: &Database) -> Edition {
        Edition { inner: attach_db(&db.db, || self.inner.edition(&db.db)) }
    }
}
