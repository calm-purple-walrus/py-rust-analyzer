use pyo3::prelude::*;

use crate::types::{AstCfgMeta, AstCfgPredicate};

#[pymethods]
impl AstCfgMeta {
    /// `pub fn cfg_predicate(&self) -> Option<CfgPredicate>`
    fn cfg_predicate(&self) -> Option<AstCfgPredicate> {
        self.inner.cfg_predicate().map(|inner| AstCfgPredicate { inner })
    }
}
