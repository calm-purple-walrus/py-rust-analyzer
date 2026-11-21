use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstFn, Function, SemanticsImpl};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn to_def<T: ToDef<'db>>(&self, src: &T) -> Option<T::Def>`, with `T` the `ast::Fn`.
    fn to_def(&self, py: Python<'_>, src: &AstFn) -> Option<Function> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.to_def(&*src.inner)).map(|inner| Function { inner })
    }
}
