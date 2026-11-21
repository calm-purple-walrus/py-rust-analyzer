use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstMeta, AstCfgAttrMeta};

#[pymethods]
impl AstCfgAttrMeta {
    /// `pub fn metas(&self) -> AstChildren<Meta>`
    fn metas(&self) -> Iter {
        Iter::new(self.inner.metas(), |py, inner| Ok(Py::new(py, AstMeta { inner })?.into_any()), None)
    }
}
