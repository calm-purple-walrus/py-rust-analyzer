use pyo3::prelude::*;
use ra_ap_hir::{HasSource, attach_db};

use crate::database::Database;
use crate::types::{Adt, InFileAstAdt};

#[pymethods]
impl Adt {
    /// `impl HasSource for Adt`: `fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Adt>>`
    fn source(&self, db: &Database) -> Option<InFileAstAdt> {
        attach_db(&db.db, || self.inner.source(&db.db)).map(|inner| InFileAstAdt { inner })
    }
}
