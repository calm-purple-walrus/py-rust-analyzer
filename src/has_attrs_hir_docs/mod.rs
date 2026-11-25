use pyo3::prelude::*;
use ra_ap_hir::{HasAttrs, attach_db};

use crate::database::Database;
use crate::types::{Adt, Const, Docs, EnumVariant, Field, Function, Impl, Static, Trait, TypeAlias};

#[pymethods]
impl Field {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl EnumVariant {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Adt {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Trait {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Impl {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Function {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Const {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl Static {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}

#[pymethods]
impl TypeAlias {
    /// `HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy.
    fn hir_docs(&self, db: &Database) -> Option<Docs> {
        attach_db(&db.db, || self.inner.hir_docs(&db.db).cloned()).map(|inner| Docs { inner })
    }
}
