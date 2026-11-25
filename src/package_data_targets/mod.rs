use pyo3::prelude::*;

use crate::types::{PackageData, Target};

#[pymethods]
impl PackageData {
    /// `pub targets: Vec<Target>`
    #[getter]
    fn targets(&self) -> Vec<Target> {
        self.inner.targets.iter().map(|&inner| Target { inner }).collect()
    }
}
