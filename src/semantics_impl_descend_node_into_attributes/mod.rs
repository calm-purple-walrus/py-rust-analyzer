use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstExpr, AstMacroCall, AstPat, AstPath, SemanticsImpl};

#[derive(FromPyObject)]
pub enum Node<'py> {
    MacroCall(PyRef<'py, AstMacroCall>),
    Expr(PyRef<'py, AstExpr>),
    Pat(PyRef<'py, AstPat>),
    Path(PyRef<'py, AstPath>),
}

#[pymethods]
impl SemanticsImpl {
    /// `fn descend_node_into_attributes<N: AstNode>(&self, node: N) -> SmallVec<[N; 1]>`
    /// It takes the node, so it is given a copy.
    fn descend_node_into_attributes(
        &self,
        py: Python<'_>,
        node: Node<'_>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || match node {
            Node::MacroCall(node) => self
                .inner
                .descend_node_into_attributes(node.inner.clone())
                .into_iter()
                .map(|inner| AstMacroCall { inner }.into_py_any(py))
                .collect(),
            Node::Expr(node) => self
                .inner
                .descend_node_into_attributes(node.inner.clone())
                .into_iter()
                .map(|inner| AstExpr { inner }.into_py_any(py))
                .collect(),
            Node::Pat(node) => self
                .inner
                .descend_node_into_attributes(node.inner.clone())
                .into_iter()
                .map(|inner| AstPat { inner }.into_py_any(py))
                .collect(),
            Node::Path(node) => self
                .inner
                .descend_node_into_attributes(node.inner.clone())
                .into_iter()
                .map(|inner| AstPath { inner }.into_py_any(py))
                .collect(),
        })
    }
}
