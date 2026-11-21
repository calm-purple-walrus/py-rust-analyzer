use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{FileId, Vfs, VfsPath};

#[pymethods]
impl Vfs {
    /// `pub fn file_path(&self, file_id: FileId) -> &VfsPath`, lent by this `Vfs`.
    fn file_path(slf: &Bound<'_, Self>, file_id: &FileId) -> VfsPath {
        let this = slf.borrow();
        // SAFETY: the path lives in this object, and no binding changes a `Vfs`.
        VfsPath { inner: unsafe { Held::lent(this.inner.file_path(file_id.inner), owner(slf)) } }
    }
}
