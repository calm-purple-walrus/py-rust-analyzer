use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Trait, Type};

#[pymethods]
impl Type {
    /// `pub fn impls_trait(&self, db: &'db dyn HirDatabase, trait_: Trait, args: &[Type<'db>]) -> bool`
    fn impls_trait(&self, db: &Database, trait_: &Trait, args: Vec<PyRef<'_, Type>>) -> bool {
        let args: Vec<_> = args.iter().map(|arg| (*arg.inner).clone()).collect();
        attach_db(&db.db, || self.inner.impls_trait(&db.db, trait_.inner, &args))
    }
}
