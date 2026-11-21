use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{Callable, Param};

#[pymethods]
impl Callable {
    /// `pub fn params(&self) -> Vec<Param<'db>>`
    fn params(&self, py: Python<'_>) -> Vec<Param> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.params()).into_iter().map(|inner| Param { inner, db: self.db.clone_ref(py) }).collect()
    }
}
