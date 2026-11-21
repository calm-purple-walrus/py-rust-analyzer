use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `impl Default for CargoConfig`
    #[staticmethod]
    fn default() -> CargoConfig {
        CargoConfig { inner: Default::default() }
    }
}
