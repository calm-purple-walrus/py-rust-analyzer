use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{Type, TypeInfo};

#[pymethods]
impl TypeInfo {
    /// `pub original: Type<'db>`, lent by this `TypeInfo`.
    #[getter]
    fn original(slf: &Bound<'_, Self>) -> Type {
        let this = slf.borrow();
        // SAFETY: the type lives in this object, and no binding changes a `TypeInfo`.
        Type { inner: unsafe { Held::lent(&this.inner.original, owner(slf)) }, db: this.db.clone_ref(slf.py()) }
    }
}
