use pyo3::prelude::*;

use crate::held::{extend, owner};
use crate::iter::Iter;
use crate::types::{CargoWorkspace, Package};

#[pymethods]
impl CargoWorkspace {
    /// `pub fn packages(&self) -> impl ExactSizeIterator<Item = Package> + '_`, lent by this
    /// `CargoWorkspace`.
    fn packages(slf: &Bound<'_, Self>) -> Iter {
        let this = slf.borrow();
        // SAFETY: the iterator borrows the workspace in this object, which it keeps alive; no binding
        // changes a `CargoWorkspace`.
        let cargo = unsafe { extend(&*this.inner) };
        Iter::new(cargo.packages(), |py, inner| Ok(Py::new(py, Package { inner })?.into_any()), Some(owner(slf)))
    }
}
