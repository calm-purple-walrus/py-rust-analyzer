use crate::types::{AstFn, AstParamList};
use pyo3::prelude::*;

#[pymethods]
impl AstFn {
    /// `fn param_list(&self) -> Option<ParamList>`
    fn param_list(&self) -> Option<AstParamList> {
        self.inner.param_list().map(|inner| AstParamList { inner })
    }
}
