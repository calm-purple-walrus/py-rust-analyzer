use pyo3::prelude::*;

use crate::held::Held;
use crate::types::CargoWorkspace;

#[pymethods]
impl CargoWorkspace {
    /// `impl Clone for CargoWorkspace`
    fn clone(&self) -> CargoWorkspace {
        CargoWorkspace { inner: Held::Own((*self.inner).clone()) }
    }
}
