use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstRecordField, AstRecordFieldList};

#[pymethods]
impl AstRecordFieldList {
    /// `pub fn fields(&self) -> AstChildren<RecordField>`
    fn fields(&self) -> Iter {
        Iter::new(self.inner.fields(), |py, inner| Ok(Py::new(py, AstRecordField { inner })?.into_any()), None)
    }
}
