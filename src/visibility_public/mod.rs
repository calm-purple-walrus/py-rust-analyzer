use pyo3::prelude::*;

use crate::types::Visibility;

#[pymethods]
impl Visibility {
    /// The variant `Visibility::Public`, made.
    #[staticmethod]
    #[pyo3(name = "Public")]
    fn public() -> Visibility {
        Visibility { inner: ra_ap_hir::Visibility::Public }
    }
}
