use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{GenericDef, TypeOrConstParam};

#[pymethods]
impl GenericDef {
    /// `pub fn type_or_const_params(self, db: &dyn HirDatabase) -> Vec<TypeOrConstParam>`
    fn type_or_const_params(&self, db: &Database) -> Vec<TypeOrConstParam> {
        attach_db(&db.db, || self.inner.type_or_const_params(&db.db))
            .into_iter()
            .map(|inner| TypeOrConstParam { inner })
            .collect()
    }
}
