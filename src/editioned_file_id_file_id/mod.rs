use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{EditionedFileId, FileId};

#[pymethods]
impl EditionedFileId {
    /// `pub fn file_id(self, db: &dyn Database) -> vfs::FileId`
    fn file_id(&self, db: &Database) -> FileId {
        FileId { inner: attach_db(&db.db, || self.inner.file_id(&db.db)) }
    }
}
