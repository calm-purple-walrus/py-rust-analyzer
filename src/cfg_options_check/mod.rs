use pyo3::prelude::*;

use crate::types::{CfgExpr, CfgOptions};

#[pymethods]
impl CfgOptions {
    /// `pub fn check(&self, cfg: &CfgExpr) -> Option<bool>`
    fn check(&self, cfg: &CfgExpr) -> Option<bool> {
        self.inner.check(&cfg.inner)
    }
}
