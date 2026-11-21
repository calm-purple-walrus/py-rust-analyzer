use pyo3::prelude::*;
use ra_ap_syntax::ast::HasName;

use crate::types::{AstAdt, AstFn, AstName};

#[pymethods]
impl AstFn {
    /// `HasName::name`: `fn name(&self) -> Option<ast::Name>`
    fn name(&self) -> Option<AstName> {
        self.inner.name().map(|inner| AstName { inner })
    }
}

#[pymethods]
impl AstAdt {
    /// `HasName::name`: `fn name(&self) -> Option<ast::Name>`
    fn name(&self) -> Option<AstName> {
        self.inner.name().map(|inner| AstName { inner })
    }
}
