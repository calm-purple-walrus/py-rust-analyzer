use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::Type;

#[pymethods]
impl Type {
    /// `pub fn is_tuple(&self) -> bool`
    fn is_tuple(&self, py: Python<'_>) -> bool {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.is_tuple())
    }
}
