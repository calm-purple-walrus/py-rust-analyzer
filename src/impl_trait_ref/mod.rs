use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::extend;
use crate::types::{Impl, TraitRef};

#[pymethods]
impl Impl {
    /// `pub fn trait_ref(self, db: &dyn HirDatabase) -> Option<TraitRef<'_>>`
    fn trait_ref(&self, py: Python<'_>, db: Py<Database>) -> Option<TraitRef> {
        // SAFETY: the `TraitRef` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || self.inner.trait_ref(database))?;
        Some(TraitRef { inner, db: db.clone_ref(py) })
    }
}
