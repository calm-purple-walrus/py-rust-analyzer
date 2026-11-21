use std::ops::Deref;

use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::{extend, owner};
use crate::types::{Semantics, SemanticsImpl};

#[pymethods]
impl Semantics {
    /// `impl Deref<Target = SemanticsImpl<'db>> for Semantics<'db, DB>`, lent by this `Semantics`.
    fn deref(slf: &Bound<'_, Self>) -> SemanticsImpl {
        let this = slf.borrow();
        let db = this.db.borrow(slf.py());
        // SAFETY: the `SemanticsImpl` lives in this object, and no binding changes a `Semantics`.
        let inner = unsafe { extend(attach_db(&db.db, || Deref::deref(&this.inner))) };
        SemanticsImpl { inner, db: this.db.clone_ref(slf.py()), _owner: owner(slf) }
    }
}
