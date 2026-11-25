use pyo3::prelude::*;

use crate::types::{CargoConfig, TargetDirectoryConfig};

#[pymethods]
impl CargoConfig {
    /// `pub target_dir_config: TargetDirectoryConfig`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn target_dir_config(&self) -> TargetDirectoryConfig {
        TargetDirectoryConfig { inner: self.inner.target_dir_config.clone() }
    }

    #[setter]
    fn set_target_dir_config(&mut self, value: &TargetDirectoryConfig) {
        self.inner.target_dir_config = value.inner.clone();
    }
}
