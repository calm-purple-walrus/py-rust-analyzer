use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::Crate;

#[pymethods]
impl Crate {
    /// `pub fn all(db: &dyn HirDatabase) -> Vec<Crate>`
    #[staticmethod]
    fn all(db: &Database) -> Vec<Crate> {
        attach_db(&db.db, || ra_ap_hir::Crate::all(&db.db)).into_iter().map(|inner| Crate { inner }).collect()
    }
}
