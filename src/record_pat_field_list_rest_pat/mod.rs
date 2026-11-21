use pyo3::prelude::*;

use crate::types::{AstRecordPatFieldList, AstRestPat};

#[pymethods]
impl AstRecordPatFieldList {
    /// `pub fn rest_pat(&self) -> Option<RestPat>`
    fn rest_pat(&self) -> Option<AstRestPat> {
        self.inner.rest_pat().map(|inner| AstRestPat { inner })
    }
}
