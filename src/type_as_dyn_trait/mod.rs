use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{Trait, Type};

#[pymethods]
impl Type {
    /// `pub fn as_dyn_trait(&self) -> Option<Trait>`
    fn as_dyn_trait(&self, py: Python<'_>) -> Option<Trait> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.as_dyn_trait()).map(|inner| Trait { inner })
    }
}
