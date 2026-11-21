use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstMacroCall, InFileSyntaxNode, SemanticsImpl};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn expand_macro_call(&self, macro_call: &ast::MacroCall) -> Option<InFile<SyntaxNode>>`
    fn expand_macro_call(&self, py: Python<'_>, macro_call: &AstMacroCall) -> Option<InFileSyntaxNode> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.expand_macro_call(&macro_call.inner)).map(|inner| InFileSyntaxNode { inner })
    }
}
