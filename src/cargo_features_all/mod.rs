use pyo3::prelude::*;

use crate::types::CargoFeatures;

#[pymethods]
impl CargoFeatures {
    /// The variant `CargoFeatures::All`, made.
    #[staticmethod]
    #[pyo3(name = "All")]
    fn all() -> CargoFeatures {
        CargoFeatures { inner: ra_ap_project_model::CargoFeatures::All }
    }
}
