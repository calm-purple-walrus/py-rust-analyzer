use crate::types::{AstClosureExpr, AstParamList};
use pyo3::prelude::*;

#[pymethods]
impl AstClosureExpr {
    /// `fn param_list(&self) -> Option<ParamList>`
    fn param_list(&self) -> Option<AstParamList> {
        self.inner.param_list().map(|inner| AstParamList { inner })
    }
}
