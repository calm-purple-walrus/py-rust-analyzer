use pyo3::prelude::*;

use crate::types::CrateDisplayName;

#[pymethods]
impl CrateDisplayName {
    /// `impl Display for CrateDisplayName`
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}
