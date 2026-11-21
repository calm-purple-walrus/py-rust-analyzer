use pyo3::prelude::*;

use crate::types::{AbsPathBuf, RustLibSource};

#[pymethods]
impl RustLibSource {
    /// The variant `RustLibSource::Path(AbsPathBuf)`, made. It takes the path, so it is given a copy.
    #[staticmethod]
    #[pyo3(name = "Path")]
    fn path(path: &AbsPathBuf) -> RustLibSource {
        RustLibSource { inner: ra_ap_project_model::RustLibSource::Path(path.inner.clone()) }
    }
}
