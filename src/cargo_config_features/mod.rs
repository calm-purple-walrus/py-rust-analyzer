use pyo3::prelude::*;

use crate::types::{CargoConfig, CargoFeatures};

#[pymethods]
impl CargoConfig {
    /// `pub features: CargoFeatures`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn features(&self) -> CargoFeatures {
        CargoFeatures { inner: self.inner.features.clone() }
    }

    #[setter]
    fn set_features(&mut self, value: &CargoFeatures) {
        self.inner.features = value.inner.clone();
    }
}
