use pyo3::prelude::*;
use ra_ap_syntax::AstToken;

use crate::types::{AstComment, SyntaxToken};

#[pymethods]
impl AstComment {
    /// `impl AstToken for Comment`: `fn cast(syntax: SyntaxToken) -> Option<Self>`. It takes the token, so it
    /// is given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxToken) -> Option<AstComment> {
        ra_ap_syntax::ast::Comment::cast((*syntax.inner).clone()).map(|inner| AstComment { inner })
    }
}
