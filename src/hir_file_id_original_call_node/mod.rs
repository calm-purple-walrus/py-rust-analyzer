use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{HirFileId, InRealFileSyntaxNode};

#[pymethods]
impl HirFileId {
    /// `fn original_call_node(self, db: &dyn SourceDatabase) -> Option<InRealFile<SyntaxNode>>`
    fn original_call_node(&self, db: &Database) -> Option<InRealFileSyntaxNode> {
        attach_db(&db.db, || self.inner.original_call_node(&db.db)).map(|inner| InRealFileSyntaxNode { inner })
    }
}
