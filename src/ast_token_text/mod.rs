use pyo3::prelude::*;
use ra_ap_syntax::AstToken;

use crate::types::AstComment;

#[pymethods]
impl AstComment {
    /// `AstToken::text`: `fn text(&self) -> &str`
    fn text(&self) -> String {
        self.inner.text().to_owned()
    }
}
