use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::{Mutability, Type};

#[pymethods]
impl Type {
    /// `pub fn as_raw_ptr(&self) -> Option<(Type<'db>, Mutability)>`
    fn as_raw_ptr(&self, py: Python<'_>) -> Option<(Type, Mutability)> {
        let db = self.db.borrow(py);
        let (inner, mutability) = attach_db(&db.db, || self.inner.as_raw_ptr())?;
        Some((Type { inner: Held::Own(inner), db: self.db.clone_ref(py) }, Mutability { inner: mutability }))
    }
}
