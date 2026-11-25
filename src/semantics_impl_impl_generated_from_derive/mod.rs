use crate::types::{Adt, Impl, SemanticsImpl};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl SemanticsImpl {
    /// `fn impl_generated_from_derive(&self, impl_: Impl) -> Option<Adt>`
    fn impl_generated_from_derive(&self, py: Python<'_>, impl_: &Impl) -> Option<Adt> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || {
            self.inner.impl_generated_from_derive(impl_.inner)
        })?;
        Some(Adt { inner })
    }
}
