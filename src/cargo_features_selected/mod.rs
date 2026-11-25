use pyo3::prelude::*;

use crate::types::CargoFeatures;

#[pymethods]
impl CargoFeatures {
    /// The variant `CargoFeatures::Selected { features: Vec<String>, no_default_features: bool }`, made.
    #[staticmethod]
    #[pyo3(name = "Selected")]
    fn selected(features: Vec<String>, no_default_features: bool) -> CargoFeatures {
        CargoFeatures { inner: ra_ap_project_model::CargoFeatures::Selected { features, no_default_features } }
    }
}
