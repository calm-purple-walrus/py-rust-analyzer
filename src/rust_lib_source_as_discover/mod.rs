use pyo3::prelude::*;

use crate::types::RustLibSource;

#[pymethods]
impl RustLibSource {
    /// The variant `RustLibSource::Discover`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Discover")]
    fn as_discover(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::RustLibSource::Discover)
    }
}
