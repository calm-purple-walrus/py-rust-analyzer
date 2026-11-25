use crate::types::{AstPat, AstRangePat};
use pyo3::prelude::*;
use ra_ap_syntax::ast::RangeItem;

#[pymethods]
impl AstRangePat {
    /// `fn start(&self) -> Option<Pat>`
    fn start(&self) -> Option<AstPat> {
        self.inner.start().map(|inner| AstPat { inner })
    }
}
