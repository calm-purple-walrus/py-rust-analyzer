use pyo3::prelude::*;
use ra_ap_hir::{HasVisibility, attach_db};

use crate::database::Database;
use crate::types::{Field, Module};

#[pymethods]
impl Field {
    /// `HasVisibility::is_visible_from`: `fn is_visible_from(&self, db: &dyn HirDatabase, module: Module) -> bool`
    fn is_visible_from(&self, db: &Database, module: &Module) -> bool {
        attach_db(&db.db, || self.inner.is_visible_from(&db.db, module.inner))
    }
}
