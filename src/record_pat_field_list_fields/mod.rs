use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstRecordPatField, AstRecordPatFieldList};

#[pymethods]
impl AstRecordPatFieldList {
    /// `pub fn fields(&self) -> AstChildren<RecordPatField>`
    fn fields(&self) -> Iter {
        Iter::new(self.inner.fields(), |py, inner| Ok(Py::new(py, AstRecordPatField { inner })?.into_any()), None)
    }
}
