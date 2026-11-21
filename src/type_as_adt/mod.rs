use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{Adt, Type};

#[pymethods]
impl Type {
    /// `pub fn as_adt(&self) -> Option<Adt>`
    fn as_adt(&self, py: Python<'_>) -> Option<Adt> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.as_adt()).map(|inner| Adt { inner })
    }
}
