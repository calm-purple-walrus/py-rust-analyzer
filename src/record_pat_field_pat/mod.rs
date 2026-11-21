use pyo3::prelude::*;

use crate::types::{AstRecordPatField, AstPat};

#[pymethods]
impl AstRecordPatField {
    /// `pub fn pat(&self) -> Option<Pat>`
    fn pat(&self) -> Option<AstPat> {
        self.inner.pat().map(|inner| AstPat { inner })
    }
}
