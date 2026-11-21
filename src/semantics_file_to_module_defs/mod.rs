use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::{extend, owner};
use crate::iter::Iter;
use crate::types::{FileId, Module, Semantics};

#[pymethods]
impl Semantics {
    /// `pub fn file_to_module_defs(&self, file: impl Into<FileId>) -> impl Iterator<Item = Module>`, with `file` a
    /// `FileId`, lent by this `Semantics`.
    fn file_to_module_defs(slf: &Bound<'_, Self>, file: &FileId) -> Iter {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the iterator borrows this object, which it keeps alive, and the database, which this
        // object keeps alive; no binding changes either.
        let semantics = unsafe { extend(&this.inner) };
        let db = unsafe { extend(&this.db.borrow(py).db) };
        let mut modules = attach_db(db, || semantics.file_to_module_defs(file.inner));
        Iter::from_fn(
            move |py| attach_db(db, || modules.next()).map(|inner| Module { inner }.into_py_any(py)).transpose(),
            Some(owner(slf)),
        )
    }
}
