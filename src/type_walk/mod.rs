use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{Held, extend};
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn walk(&self, db: &'db dyn HirDatabase, callback: impl FnMut(Type<'db>))`
    ///
    /// `callback` is a Python callable that takes a `Type`. If it raises, the binding panics.
    fn walk(&self, py: Python<'_>, db: Py<Database>, callback: Py<PyAny>) {
        // SAFETY: every `Type` given to `callback` holds `db`, and `Database` offers no way to change
        // or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        attach_db(database, || {
            self.inner.walk(database, |inner| {
                let ty = Type { inner: Held::Own(inner), db: db.clone_ref(py) };
                if let Err(error) = callback.call1(py, (ty,)) {
                    panic!("the walk callback raised {error}");
                }
            })
        });
    }
}
