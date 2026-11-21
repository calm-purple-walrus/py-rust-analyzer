use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::{Held, extend, owner};
use crate::iter::Iter;
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn type_arguments(&self) -> impl Iterator<Item = Type<'db>> + '_`, lent by this `Type`.
    fn type_arguments(slf: &Bound<'_, Self>) -> Iter {
        let py = slf.py();
        let this = slf.borrow();
        let db = this.db.clone_ref(py);
        // SAFETY: the iterator borrows this object, which it keeps alive, and the database, which this object
        // keeps alive; no binding changes either.
        let ty = unsafe { extend(&*this.inner) };
        let database = unsafe { extend(&this.db.borrow(py).db) };
        let mut arguments = attach_db(database, || ty.type_arguments());
        Iter::from_fn(
            move |py| {
                attach_db(database, || arguments.next())
                    .map(|inner| Type { inner: Held::Own(inner), db: db.clone_ref(py) }.into_py_any(py))
                    .transpose()
            },
            Some(owner(slf)),
        )
    }
}
