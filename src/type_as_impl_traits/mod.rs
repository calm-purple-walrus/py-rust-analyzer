use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::held::{extend, owner};
use crate::iter::Iter;
use crate::types::{Trait, Type};

#[pymethods]
impl Type {
    /// `pub fn as_impl_traits(&self, db: &'db dyn HirDatabase) -> Option<impl Iterator<Item = Trait>>`, the iterator
    /// lent by this `Type`.
    fn as_impl_traits(slf: &Bound<'_, Self>, db: Py<Database>) -> Option<Iter> {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the iterator borrows this object, which it keeps alive, and the database, whose Python object it
        // holds; no binding changes either.
        let ty = unsafe { extend(&*this.inner) };
        let database = unsafe { extend(&db.borrow(py).db) };
        let mut traits = attach_db(database, || ty.as_impl_traits(database))?;
        Some(Iter::from_fn(
            move |py| {
                let _ = &db;
                attach_db(database, || traits.next()).map(|inner| Trait { inner }.into_py_any(py)).transpose()
            },
            Some(owner(slf)),
        ))
    }
}
