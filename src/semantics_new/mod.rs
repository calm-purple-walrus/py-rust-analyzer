use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::extend;
use crate::types::Semantics;

#[pymethods]
impl Semantics {
    /// `pub fn new(db: &DB) -> Semantics<'_, DB>`, with `DB` the `RootDatabase`.
    #[staticmethod]
    fn new(py: Python<'_>, db: Py<Database>) -> Semantics {
        // SAFETY: the `Semantics` holds `db`, and `Database` offers no way to change or move the database.
        let database = unsafe { extend(&db.borrow(py).db) };
        Semantics { inner: attach_db(database, || ra_ap_hir::Semantics::new(database)), db: db.clone_ref(py) }
    }
}
