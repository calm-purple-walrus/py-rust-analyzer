use crate::types::{AstPat, AstRangePat};
use pyo3::prelude::*;
use ra_ap_syntax::ast::RangeItem;

#[pymethods]
impl AstRangePat {
    /// `fn end(&self) -> Option<Pat>`
    fn end(&self) -> Option<AstPat> {
        self.inner.end().map(|inner| AstPat { inner })
    }
}
