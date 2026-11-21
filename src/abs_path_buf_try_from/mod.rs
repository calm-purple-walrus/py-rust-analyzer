use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::types::AbsPathBuf;

#[pymethods]
impl AbsPathBuf {
    /// `impl TryFrom<&str> for AbsPathBuf`: `fn try_from(path: &str) -> Result<AbsPathBuf, Utf8PathBuf>`
    #[staticmethod]
    #[pyo3(name = "try_from_str")]
    fn try_from(value: &str) -> PyResult<AbsPathBuf> {
        ra_ap_paths::AbsPathBuf::try_from(value)
            .map(|inner| AbsPathBuf { inner })
            .map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))
    }
}
