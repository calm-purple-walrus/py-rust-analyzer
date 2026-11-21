use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{TypeOrConstParam, TypeParam};

#[pymethods]
impl TypeOrConstParam {
    /// `pub fn as_type_param(self, db: &dyn HirDatabase) -> Option<TypeParam>`
    fn as_type_param(&self, db: &Database) -> Option<TypeParam> {
        attach_db(&db.db, || self.inner.as_type_param(&db.db)).map(|inner| TypeParam { inner })
    }
}
