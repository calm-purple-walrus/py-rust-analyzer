use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Enum, EnumVariant};

#[pymethods]
impl EnumVariant {
    /// `pub fn parent_enum(self, db: &dyn HirDatabase) -> Enum`
    fn parent_enum(&self, db: &Database) -> Enum {
        Enum { inner: attach_db(&db.db, || self.inner.parent_enum(&db.db)) }
    }
}
