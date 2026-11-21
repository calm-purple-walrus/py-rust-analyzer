use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Trait, TypeParam};

#[pymethods]
impl TypeParam {
    /// `pub fn trait_bounds(self, db: &dyn HirDatabase) -> Vec<Trait>`
    fn trait_bounds(&self, db: &Database) -> Vec<Trait> {
        attach_db(&db.db, || self.inner.trait_bounds(&db.db)).into_iter().map(|inner| Trait { inner }).collect()
    }
}
