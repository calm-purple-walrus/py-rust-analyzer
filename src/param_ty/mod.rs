use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::{Held, owner};
use crate::types::{Param, Type};

#[pymethods]
impl Param {
    /// `pub fn ty(&self) -> &Type<'db>`, lent by this `Param`.
    fn ty(slf: &Bound<'_, Self>) -> Type {
        let this = slf.borrow();
        let db = this.db.borrow(slf.py());
        // SAFETY: the type lives in this object, and no binding changes a `Param`.
        Type { inner: unsafe { Held::lent(attach_db(&db.db, || this.inner.ty()), owner(slf)) }, db: this.db.clone_ref(slf.py()) }
    }
}
