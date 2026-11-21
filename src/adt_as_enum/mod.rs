use pyo3::prelude::*;

use crate::types::{Adt, Enum};

#[pymethods]
impl Adt {
    /// `pub fn as_enum(&self) -> Option<Enum>`
    fn as_enum(&self) -> Option<Enum> {
        self.inner.as_enum().map(|inner| Enum { inner })
    }
}
