use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn future_output(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>`. It takes the type, so it is
    /// given a copy.
    fn future_output(&self, py: Python<'_>, db: Py<Database>) -> Option<Type> {
        // SAFETY: the `Type` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || (*self.inner).clone().future_output(database))?;
        Some(Type { inner: Held::Own(inner), db: db.clone_ref(py) })
    }
}
