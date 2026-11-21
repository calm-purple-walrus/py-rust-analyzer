use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{FileRange, SemanticsImpl, SyntaxNode};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn original_range_opt(&self, node: &SyntaxNode) -> Option<FileRange>`
    fn original_range_opt(&self, py: Python<'_>, node: &SyntaxNode) -> Option<FileRange> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.original_range_opt(&node.inner)).map(|inner| FileRange { inner })
    }
}
