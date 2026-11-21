use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::{Function, Type};

#[pymethods]
impl Function {
    /// `pub fn ret_type(self, db: &dyn HirDatabase) -> Type<'_>`
    fn ret_type(&self, py: Python<'_>, db: Py<Database>) -> Type {
        // SAFETY: the `Type` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        Type { inner: Held::Own(attach_db(database, || self.inner.ret_type(database))), db: db.clone_ref(py) }
    }
}
