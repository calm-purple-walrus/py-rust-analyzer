use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{FileRange, InFileTextRange};

#[pymethods]
impl InFileTextRange {
    /// `impl InFile<TextRange>`: `pub fn original_node_file_range_rooted(self, db: &dyn SourceDatabase) -> FileRange`
    fn original_node_file_range_rooted(&self, db: &Database) -> FileRange {
        FileRange { inner: attach_db(&db.db, || self.inner.original_node_file_range_rooted(&db.db)) }
    }
}
