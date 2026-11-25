use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{extend, owner};
use crate::iter::Iter;
use crate::types::{DisplayTarget, Type};

#[pymethods]
impl Type {
    /// `pub fn type_and_const_arguments<'a>(&'a self, db: &'a dyn HirDatabase, display_target: DisplayTarget) -> impl Iterator<Item = SmolStr> + 'a`,
    /// lent by this `Type` and the database.
    fn type_and_const_arguments(slf: &Bound<'_, Self>, db: Py<Database>, display_target: &DisplayTarget) -> Iter {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the iterator borrows this object, which it keeps alive, and `db`, which its closure keeps
        // alive; no binding changes either.
        let ty = unsafe { extend(&*this.inner) };
        let database = unsafe { extend(&db.borrow(py).db) };
        let target = display_target.inner;
        let mut arguments = attach_db(database, || ty.type_and_const_arguments(database, target));
        Iter::from_fn(
            move |py| {
                let _held = &db;
                attach_db(database, || arguments.next()).map(|text| text.to_string().into_py_any(py)).transpose()
            },
            Some(owner(slf)),
        )
    }
}
