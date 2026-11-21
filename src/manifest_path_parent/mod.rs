use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AbsPath, ManifestPath};

#[pymethods]
impl ManifestPath {
    /// `pub fn parent(&self) -> &AbsPath`, lent by this `ManifestPath`.
    fn parent(slf: &Bound<'_, Self>) -> AbsPath {
        let this = slf.borrow();
        // SAFETY: the path lives in this object, and no binding changes a `ManifestPath`.
        AbsPath { inner: unsafe { Held::lent(this.inner.parent(), owner(slf)) } }
    }
}
