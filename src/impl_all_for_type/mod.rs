use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Impl, Type};

#[pymethods]
impl Impl {
    /// `pub fn all_for_type<'db>(db: &'db dyn HirDatabase, ty: Type<'db>) -> Vec<Impl>`. It takes the type, so it is
    /// given a copy.
    #[staticmethod]
    fn all_for_type(db: &Database, ty: &Type) -> Vec<Impl> {
        attach_db(&db.db, || ra_ap_hir::Impl::all_for_type(&db.db, (*ty.inner).clone())).into_iter().map(|inner| Impl { inner }).collect()
    }
}
