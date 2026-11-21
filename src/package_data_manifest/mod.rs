use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{ManifestPath, PackageData};

#[pymethods]
impl PackageData {
    /// `pub manifest: ManifestPath`, lent by this `PackageData`.
    #[getter]
    fn manifest(slf: &Bound<'_, Self>) -> ManifestPath {
        let this = slf.borrow();
        // SAFETY: the manifest path lives in this object, and no binding changes a `PackageData`.
        ManifestPath { inner: unsafe { Held::lent(&this.inner.manifest, owner(slf)) } }
    }
}
