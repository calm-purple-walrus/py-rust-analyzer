use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn tuple_fields(&self, _db: &'db dyn HirDatabase) -> Vec<Self>`
    fn tuple_fields(&self, py: Python<'_>, db: Py<Database>) -> Vec<Type> {
        // SAFETY: each `Type` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        attach_db(database, || self.inner.tuple_fields(database))
            .into_iter()
            .map(|inner| Type { inner: Held::Own(inner), db: db.clone_ref(py) })
            .collect()
    }
}
