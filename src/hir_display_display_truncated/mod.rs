use pyo3::prelude::*;
use ra_ap_hir::{HirDisplay, attach_db};

use crate::database::Database;
use crate::held::{extend, owner};
use crate::types::{DisplayTarget, HirDisplayWrapper, Type};

#[pymethods]
impl Type {
    /// `impl HirDisplay for Type`: `fn display_truncated<'a>(&'a self, db: &'db dyn HirDatabase, max_size: Option<usize>, display_target: DisplayTarget) -> HirDisplayWrapper<'a, 'db, Self>`,
    /// lent by this `Type` and the database.
    fn display_truncated(
        slf: &Bound<'_, Self>,
        db: Py<Database>,
        max_size: Option<usize>,
        display_target: &DisplayTarget,
    ) -> HirDisplayWrapper {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the wrapper holds this object and `db`, and no binding changes a `Type` or a
        // `Database`.
        let ty = unsafe { extend(&*this.inner) };
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || ty.display_truncated(database, max_size, display_target.inner));
        HirDisplayWrapper { inner, db: db.clone_ref(py), _owner: owner(slf) }
    }
}
