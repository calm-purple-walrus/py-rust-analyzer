use pyo3::prelude::*;

use crate::types::AbsPath;

#[pymethods]
impl AbsPath {
    /// `impl Display for AbsPath`
    fn __str__(&self) -> String {
        let path: &ra_ap_paths::AbsPath = &self.inner;
        path.to_string()
    }
}
