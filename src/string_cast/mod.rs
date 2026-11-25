use pyo3::prelude::*;
use ra_ap_syntax::AstToken;

use crate::types::{AstString, SyntaxToken};

#[pymethods]
impl AstString {
    /// `AstToken::cast(syntax: SyntaxToken) -> Option<String>`. It takes a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxToken) -> Option<Self> {
        ra_ap_syntax::ast::String::cast((*syntax.inner).clone()).map(|inner| AstString { inner })
    }
}
