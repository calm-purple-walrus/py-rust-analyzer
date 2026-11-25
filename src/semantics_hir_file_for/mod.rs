use crate::types::{HirFileId, Semantics, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl Semantics {
    /// `fn hir_file_for(&self, syntax_node: &SyntaxNode) -> HirFileId`
    fn hir_file_for(&self, py: Python<'_>, syntax_node: &SyntaxNode) -> HirFileId {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.hir_file_for(&syntax_node.inner));
        HirFileId { inner }
    }
}
