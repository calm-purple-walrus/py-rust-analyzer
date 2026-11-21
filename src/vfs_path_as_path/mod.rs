use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AbsPath, VfsPath};

#[pymethods]
impl VfsPath {
    /// `pub fn as_path(&self) -> Option<&AbsPath>`, lent by this `VfsPath`.
    fn as_path(slf: &Bound<'_, Self>) -> Option<AbsPath> {
        let this = slf.borrow();
        // SAFETY: the path lives in this object, and no binding changes a `VfsPath`.
        this.inner.as_path().map(|path| AbsPath { inner: unsafe { Held::lent(path, owner(slf)) } })
    }
}
