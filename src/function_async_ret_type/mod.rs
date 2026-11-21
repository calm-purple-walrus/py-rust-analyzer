use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::{Function, Type};

#[pymethods]
impl Function {
    /// `pub fn async_ret_type<'db>(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>`
    fn async_ret_type(&self, py: Python<'_>, db: Py<Database>) -> Option<Type> {
        // SAFETY: the `Type` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || self.inner.async_ret_type(database))?;
        Some(Type { inner: Held::Own(inner), db: db.clone_ref(py) })
    }
}
