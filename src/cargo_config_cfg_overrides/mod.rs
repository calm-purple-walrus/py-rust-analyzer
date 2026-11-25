use pyo3::prelude::*;

use crate::types::{CargoConfig, CfgOverrides};

#[pymethods]
impl CargoConfig {
    /// `pub cfg_overrides: CfgOverrides`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn cfg_overrides(&self) -> CfgOverrides {
        CfgOverrides { inner: self.inner.cfg_overrides.clone() }
    }

    #[setter]
    fn set_cfg_overrides(&mut self, value: &CfgOverrides) {
        self.inner.cfg_overrides = value.inner.clone();
    }
}
