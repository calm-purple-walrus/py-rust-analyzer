use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{FileRange, InFileSyntaxNode};

#[pymethods]
impl InFileSyntaxNode {
    /// `impl<SN: Borrow<SyntaxNode>> InFile<SN>`: `pub fn original_file_range_rooted(self, db: &dyn SourceDatabase) -> FileRange`.
    /// It takes the value, so it is given a copy.
    fn original_file_range_rooted(&self, db: &Database) -> FileRange {
        FileRange { inner: attach_db(&db.db, || self.inner.clone().original_file_range_rooted(&db.db)) }
    }
}
