use std::ops::Index;

use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{CargoWorkspace, Package, PackageData};

#[pymethods]
impl CargoWorkspace {
    /// `impl Index<Package> for CargoWorkspace`: `fn index(&self, index: Package) -> &PackageData`,
    /// lent by this `CargoWorkspace`.
    #[pyo3(name = "index_Package")]
    fn index(slf: &Bound<'_, Self>, index: &Package) -> PackageData {
        let this = slf.borrow();
        // SAFETY: the data lives in this object, and no binding changes a `CargoWorkspace`.
        PackageData { inner: unsafe { Held::lent(this.inner.index(index.inner), owner(slf)) } }
    }
}
