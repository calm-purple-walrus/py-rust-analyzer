use pyo3::prelude::*;

use crate::types::{AstPath, AstTokenTree, Edition};

/// `pub fn parse_tt_as_comma_sep_paths(input: ast::TokenTree, edition: Edition) -> Option<Vec<ast::Path>>`. It
/// takes the token tree, so it is given a copy.
#[pyfunction]
pub fn parse_tt_as_comma_sep_paths(input: &AstTokenTree, edition: &Edition) -> Option<Vec<AstPath>> {
    ra_ap_ide_db::syntax_helpers::node_ext::parse_tt_as_comma_sep_paths(input.inner.clone(), edition.inner)
        .map(|paths| paths.into_iter().map(|inner| AstPath { inner }).collect())
}
