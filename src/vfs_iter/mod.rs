use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;

use crate::held::{Held, extend, owner};
use crate::iter::Iter;
use crate::types::{FileId, Vfs, VfsPath};

#[pymethods]
impl Vfs {
    /// `pub fn iter(&self) -> impl Iterator<Item = (FileId, &VfsPath)> + '_`, lent by this `Vfs`.
    fn iter(slf: &Bound<'_, Self>) -> Iter {
        let this = slf.borrow();
        // SAFETY: the iterator and its paths borrow the `Vfs` in this object, which they keep alive;
        // no binding changes a `Vfs`.
        let vfs = unsafe { extend(&this.inner) };
        let lender = owner(slf);
        Iter::new(
            vfs.iter(),
            move |py, (file_id, path)| {
                let path = VfsPath { inner: unsafe { Held::lent(path, lender.clone_ref(py)) } };
                (FileId { inner: file_id }, path).into_py_any(py)
            },
            Some(owner(slf)),
        )
    }
}
