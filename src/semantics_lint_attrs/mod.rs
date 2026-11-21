use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::{extend, owner};
use crate::iter::Iter;
use crate::types::{AnyHasAttrs, Crate, FileId, LintAttr, Semantics};

#[pymethods]
impl Semantics {
    /// `pub fn lint_attrs(&self, file_id: FileId, krate: Crate, item: ast::AnyHasAttrs) -> impl DoubleEndedIterator<Item = (LintAttr, SmolStr)>`,
    /// lent by this `Semantics`.
    fn lint_attrs(slf: &Bound<'_, Self>, file_id: &FileId, krate: &Crate, item: &AnyHasAttrs) -> Iter {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the iterator borrows this object, which it keeps alive, and the database, which this
        // object keeps alive; no binding changes either.
        let semantics = unsafe { extend(&this.inner) };
        let db = unsafe { extend(&this.db.borrow(py).db) };
        let mut attrs = attach_db(db, || semantics.lint_attrs(file_id.inner, krate.inner, item.inner.clone()));
        Iter::from_fn(
            move |py| {
                attach_db(db, || attrs.next())
                    .map(|(attr, name)| (LintAttr { inner: attr }, name.to_string()).into_py_any(py))
                    .transpose()
            },
            Some(owner(slf)),
        )
    }
}
