use crate::types::{InFileAstName, LocalSource};
use pyo3::prelude::*;

#[pymethods]
impl LocalSource {
    /// `fn name(&self) -> Option<InFile<ast::Name>>`
    fn name(&self) -> Option<InFileAstName> {
        self.inner.name().map(|inner| InFileAstName { inner })
    }
}
