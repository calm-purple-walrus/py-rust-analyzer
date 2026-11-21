use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{PackageData, PackageDependency};

#[pymethods]
impl PackageData {
    /// `pub dependencies: Vec<PackageDependency>`, each lent by this `PackageData`.
    #[getter]
    fn dependencies(slf: &Bound<'_, Self>) -> Vec<PackageDependency> {
        let this = slf.borrow();
        // SAFETY: the dependencies live in this object, and no binding changes a `PackageData`.
        this.inner.dependencies.iter().map(|dependency| PackageDependency { inner: unsafe { Held::lent(dependency, owner(slf)) } }).collect()
    }
}
