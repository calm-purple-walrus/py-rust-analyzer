use std::ops::Deref;

use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AbsPath, AbsPathBuf};

#[pymethods]
impl AbsPathBuf {
    /// `impl Deref<Target = AbsPath> for AbsPathBuf`, lent by this `AbsPathBuf`.
    fn deref(slf: &Bound<'_, Self>) -> AbsPath {
        let this = slf.borrow();
        // SAFETY: the path lives in this object, and no binding changes an `AbsPathBuf`.
        AbsPath { inner: unsafe { Held::lent(Deref::deref(&this.inner), owner(slf)) } }
    }
}
