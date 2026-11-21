use pyo3::prelude::*;

use crate::types::{CargoConfig, RustLibSource};

#[pymethods]
impl CargoConfig {
    /// `pub sysroot: Option<RustLibSource>`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn sysroot(&self) -> Option<RustLibSource> {
        self.inner.sysroot.clone().map(|inner| RustLibSource { inner })
    }

    #[setter]
    fn set_sysroot(&mut self, value: Option<&RustLibSource>) {
        self.inner.sysroot = value.map(|source| source.inner.clone());
    }
}
