use crate::database::Database;
use crate::held::extend;
use crate::types::{Function, Param};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl Function {
    /// `fn assoc_fn_params(self, db: &dyn HirDatabase) -> Vec<Param<'_>>`
    fn assoc_fn_params(&self, py: Python<'_>, db: Py<Database>) -> Vec<Param> {
        // SAFETY: each parameter keeps this immutable database alive.
        let database = unsafe { extend(&db.borrow(py).db) };
        attach_db(database, || self.inner.assoc_fn_params(database))
            .into_iter()
            .map(|inner| Param {
                inner,
                db: db.clone_ref(py),
            })
            .collect()
    }
}
