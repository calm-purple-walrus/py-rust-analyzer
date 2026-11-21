use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn as_slice(&self) -> Option<Type<'db>>`
    fn as_slice(&self, py: Python<'_>) -> Option<Type> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.as_slice())?;
        Some(Type { inner: Held::Own(inner), db: self.db.clone_ref(py) })
    }
}
