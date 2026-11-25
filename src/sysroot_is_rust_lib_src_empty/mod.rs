use pyo3::prelude::*;

use crate::types::Sysroot;

#[pymethods]
impl Sysroot {
    /// `pub fn is_rust_lib_src_empty(&self) -> bool`
    fn is_rust_lib_src_empty(&self) -> bool {
        self.inner.is_rust_lib_src_empty()
    }
}
