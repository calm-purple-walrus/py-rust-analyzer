use pyo3::prelude::*;

use crate::types::{AstRecordPat, AstRecordPatFieldList};

#[pymethods]
impl AstRecordPat {
    /// `pub fn record_pat_field_list(&self) -> Option<RecordPatFieldList>`
    fn record_pat_field_list(&self) -> Option<AstRecordPatFieldList> {
        self.inner.record_pat_field_list().map(|inner| AstRecordPatFieldList { inner })
    }
}
