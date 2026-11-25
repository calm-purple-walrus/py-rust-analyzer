use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstPat, SyntaxNode};

#[pymethods]
impl AstPat {
    /// `impl AstNode for Pat`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a
    /// copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstPat> {
        ra_ap_syntax::ast::Pat::cast((*syntax.inner).clone()).map(|inner| AstPat { inner })
    }
}
