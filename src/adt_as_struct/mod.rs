use pyo3::prelude::*;

use crate::types::{Adt, Struct};

#[pymethods]
impl Adt {
    /// `pub fn as_struct(&self) -> Option<Struct>`
    fn as_struct(&self) -> Option<Struct> {
        self.inner.as_struct().map(|inner| Struct { inner })
    }
}
