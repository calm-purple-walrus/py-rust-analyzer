use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::{TraitRef, Type};

#[pymethods]
impl TraitRef {
    /// `pub fn get_type_argument(&self, idx: usize) -> Option<Type<'db>>`
    fn get_type_argument(&self, py: Python<'_>, idx: usize) -> Option<Type> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.get_type_argument(idx))?;
        Some(Type { inner: Held::Own(inner), db: self.db.clone_ref(py) })
    }
}
