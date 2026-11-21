use pyo3::prelude::*;
use ra_ap_hir::{HasSource, attach_db};

use crate::database::Database;
use crate::types::{Function, InFileFn};

#[pymethods]
impl Function {
    /// `impl HasSource for Function`: `fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Fn>>`
    fn source(&self, db: &Database) -> Option<InFileFn> {
        attach_db(&db.db, || self.inner.source(&db.db)).map(|inner| InFileFn { inner })
    }
}
