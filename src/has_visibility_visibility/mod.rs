use pyo3::prelude::*;
use ra_ap_hir::{HasVisibility, attach_db};

use crate::database::Database;
use crate::types::{Adt, Const, Field, Function, Static, Trait, TypeAlias, Visibility};

#[pymethods]
impl Field {
    /// `impl HasVisibility for Field`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl Adt {
    /// `impl HasVisibility for Adt`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl Trait {
    /// `impl HasVisibility for Trait`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl Function {
    /// `impl HasVisibility for Function`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl Const {
    /// `impl HasVisibility for Const`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl Static {
    /// `impl HasVisibility for Static`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}

#[pymethods]
impl TypeAlias {
    /// `impl HasVisibility for TypeAlias`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`
    fn visibility(&self, db: &Database) -> Visibility {
        Visibility { inner: attach_db(&db.db, || self.inner.visibility(&db.db)) }
    }
}
