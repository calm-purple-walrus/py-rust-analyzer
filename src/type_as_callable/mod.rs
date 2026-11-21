use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::extend;
use crate::types::{Callable, Type};

#[pymethods]
impl Type {
    /// `pub fn as_callable(&self, db: &'db dyn HirDatabase) -> Option<Callable<'db>>`
    fn as_callable(&self, py: Python<'_>, db: Py<Database>) -> Option<Callable> {
        // SAFETY: the `Callable` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || self.inner.as_callable(database))?;
        Some(Callable { inner, db: db.clone_ref(py) })
    }
}
