use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{DepKind, PackageDependency};

#[pymethods]
impl PackageDependency {
    /// `pub kind: DepKind`, lent by this `PackageDependency`.
    #[getter]
    fn kind(slf: &Bound<'_, Self>) -> DepKind {
        let this = slf.borrow();
        // SAFETY: the dependency lives in this object, and no binding changes a `PackageDependency`.
        DepKind { dependency: unsafe { Held::lent(&*this.inner, owner(slf)) } }
    }
}
