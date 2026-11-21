use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::Type;

#[pymethods]
impl Type {
    /// `impl Clone for Type<'db>`
    fn clone(&self, py: Python<'_>) -> Type {
        let db = self.db.borrow(py);
        Type { inner: Held::Own(attach_db(&db.db, || (*self.inner).clone())), db: self.db.clone_ref(py) }
    }
}
