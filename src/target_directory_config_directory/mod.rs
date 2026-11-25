use pyo3::prelude::*;

use crate::types::TargetDirectoryConfig;

#[pymethods]
impl TargetDirectoryConfig {
    /// The variant `TargetDirectoryConfig::Directory(Utf8PathBuf)`, made from the path as a string.
    #[staticmethod]
    #[pyo3(name = "Directory")]
    fn directory(path: &str) -> TargetDirectoryConfig {
        TargetDirectoryConfig { inner: ra_ap_project_model::TargetDirectoryConfig::Directory(path.into()) }
    }
}
