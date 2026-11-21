use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Crate, CrateDisplayName};

#[pymethods]
impl Crate {
    /// `pub fn display_name(self, db: &dyn HirDatabase) -> Option<CrateDisplayName>`
    fn display_name(&self, db: &Database) -> Option<CrateDisplayName> {
        attach_db(&db.db, || self.inner.display_name(&db.db)).map(|inner| CrateDisplayName { inner })
    }
}
