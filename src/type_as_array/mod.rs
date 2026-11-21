use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn as_array(&self, db: &'db dyn HirDatabase) -> Option<(Self, usize)>`
    fn as_array(&self, py: Python<'_>, db: Py<Database>) -> Option<(Type, usize)> {
        // SAFETY: the `Type` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        let (inner, length) = attach_db(database, || self.inner.as_array(database))?;
        Some((Type { inner: Held::Own(inner), db: db.clone_ref(py) }, length))
    }
}
