use pyo3::prelude::*;
use ra_ap_hir::{HirDisplay, attach_db};

use crate::database::Database;
use crate::held::{extend, owner};
use crate::types::{DisplayTarget, HirDisplayWrapperTraitRef, TraitRef};

#[pymethods]
impl TraitRef {
    /// `impl HirDisplay for TraitRef`: `fn display<'a>(&'a self, db: &'db dyn HirDatabase, display_target: DisplayTarget) -> HirDisplayWrapper<'a, 'db, Self>`,
    /// lent by this `TraitRef` and the database.
    fn display(slf: &Bound<'_, Self>, db: Py<Database>, display_target: &DisplayTarget) -> HirDisplayWrapperTraitRef {
        let py = slf.py();
        let this = slf.borrow();
        // SAFETY: the wrapper holds this object and `db`, and no binding changes a `TraitRef` or a
        // `Database`.
        let trait_ref = unsafe { extend(&this.inner) };
        let database = unsafe { extend(&db.borrow(py).db) };
        let inner = attach_db(database, || trait_ref.display(database, display_target.inner));
        HirDisplayWrapperTraitRef { inner, db: db.clone_ref(py), _owner: owner(slf) }
    }
}
