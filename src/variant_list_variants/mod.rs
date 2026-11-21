use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstVariant, AstVariantList};

#[pymethods]
impl AstVariantList {
    /// `pub fn variants(&self) -> AstChildren<Variant>`
    fn variants(&self) -> Iter {
        Iter::new(self.inner.variants(), |py, inner| Ok(Py::new(py, AstVariant { inner })?.into_any()), None)
    }
}
