use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{Module, InFileAstModule};

#[pymethods]
impl Module {
    /// `pub fn declaration_source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Module>>`
    fn declaration_source(&self, db: &Database) -> Option<InFileAstModule> {
        attach_db(&db.db, || self.inner.declaration_source(&db.db)).map(|inner| InFileAstModule { inner })
    }
}
