use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::HirDisplayWrapper;

#[pymethods]
impl HirDisplayWrapper {
    /// `impl Display for HirDisplayWrapper<'_, '_, T>`
    fn __str__(&self, py: Python<'_>) -> String {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.to_string())
    }
}
