use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AbsPath, TargetData};

#[pymethods]
impl TargetData {
    /// `pub root: AbsPathBuf`, as the `AbsPath` it derefs to, lent by this `TargetData`.
    #[getter]
    fn root(slf: &Bound<'_, Self>) -> AbsPath {
        let this = slf.borrow();
        // SAFETY: the path lives in the value this object holds or borrows, and no binding changes a `TargetData`.
        AbsPath { inner: unsafe { Held::lent(&*this.inner.root, owner(slf)) } }
    }
}
