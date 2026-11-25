use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::{AstAdt, AstFn, AstImpl, AstModule, AstRecordField, AstTrait, AstTupleField, AstVariant, SemanticsImpl};

/// The nodes `original_ast_node` is bound for.
#[derive(FromPyObject)]
pub enum Node<'py> {
    Fn(PyRef<'py, AstFn>),
    Adt(PyRef<'py, AstAdt>),
    Impl(PyRef<'py, AstImpl>),
    Trait(PyRef<'py, AstTrait>),
    Module(PyRef<'py, AstModule>),
    RecordField(PyRef<'py, AstRecordField>),
    TupleField(PyRef<'py, AstTupleField>),
    Variant(PyRef<'py, AstVariant>),
}

#[pymethods]
impl SemanticsImpl {
    /// `pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`,
    /// an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an
    /// `ast::Variant`. It takes the node, so it is given a copy.
    fn original_ast_node(&self, py: Python<'_>, node: Node<'_>) -> PyResult<Option<Py<PyAny>>> {
        let db = self.db.borrow(py);
        let original = &self.inner;
        attach_db(&db.db, || match node {
            Node::Fn(node) => original
                .original_ast_node((*node.inner).clone())
                .map(|inner| AstFn { inner: Held::Own(inner) }.into_py_any(py)),
            Node::Adt(node) => original
                .original_ast_node((*node.inner).clone())
                .map(|inner| AstAdt { inner: Held::Own(inner) }.into_py_any(py)),
            Node::Impl(node) => original
                .original_ast_node((*node.inner).clone())
                .map(|inner| AstImpl { inner: Held::Own(inner) }.into_py_any(py)),
            Node::Trait(node) => original
                .original_ast_node((*node.inner).clone())
                .map(|inner| AstTrait { inner: Held::Own(inner) }.into_py_any(py)),
            Node::Module(node) => original
                .original_ast_node((*node.inner).clone())
                .map(|inner| AstModule { inner: Held::Own(inner) }.into_py_any(py)),
            Node::RecordField(node) => {
                original.original_ast_node(node.inner.clone()).map(|inner| AstRecordField { inner }.into_py_any(py))
            }
            Node::TupleField(node) => {
                original.original_ast_node(node.inner.clone()).map(|inner| AstTupleField { inner }.into_py_any(py))
            }
            Node::Variant(node) => {
                original.original_ast_node(node.inner.clone()).map(|inner| AstVariant { inner }.into_py_any(py))
            }
        })
        .transpose()
    }
}
