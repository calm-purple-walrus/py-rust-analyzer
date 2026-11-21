use std::ops::Index;

use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{CargoWorkspace, Target, TargetData};

#[pymethods]
impl CargoWorkspace {
    /// `impl Index<Target> for CargoWorkspace`: `fn index(&self, index: Target) -> &TargetData`,
    /// lent by this `CargoWorkspace`.
    #[pyo3(name = "index_Target")]
    fn index_target(slf: &Bound<'_, Self>, index: &Target) -> TargetData {
        let this = slf.borrow();
        // SAFETY: the data lives in this object, and no binding changes a `CargoWorkspace`.
        TargetData { inner: unsafe { Held::lent(this.inner.index(index.inner), owner(slf)) } }
    }
}
