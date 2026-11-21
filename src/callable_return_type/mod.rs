use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::{Callable, Type};

#[pymethods]
impl Callable {
    /// `pub fn return_type(&self) -> Type<'db>`
    fn return_type(&self, py: Python<'_>) -> Type {
        let db = self.db.borrow(py);
        Type { inner: Held::Own(attach_db(&db.db, || self.inner.return_type())), db: self.db.clone_ref(py) }
    }
}
