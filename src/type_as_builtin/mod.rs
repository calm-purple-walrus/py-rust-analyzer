use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{BuiltinType, Type};

#[pymethods]
impl Type {
    /// `pub fn as_builtin(&self) -> Option<BuiltinType>`
    fn as_builtin(&self, py: Python<'_>) -> Option<BuiltinType> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.as_builtin()).map(|inner| BuiltinType { inner })
    }
}
