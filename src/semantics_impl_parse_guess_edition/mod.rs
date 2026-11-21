use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{FileId, SemanticsImpl, SourceFile};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn parse_guess_edition(&self, file_id: FileId) -> ast::SourceFile`
    fn parse_guess_edition(&self, py: Python<'_>, file_id: &FileId) -> SourceFile {
        let db = self.db.borrow(py);
        SourceFile { inner: attach_db(&db.db, || self.inner.parse_guess_edition(file_id.inner)) }
    }
}
