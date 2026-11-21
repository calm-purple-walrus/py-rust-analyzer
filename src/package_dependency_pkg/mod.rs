use pyo3::prelude::*;

use crate::types::{Package, PackageDependency};

#[pymethods]
impl PackageDependency {
    /// `pub pkg: Package`
    #[getter]
    fn pkg(&self) -> Package {
        Package { inner: self.inner.pkg }
    }
}
