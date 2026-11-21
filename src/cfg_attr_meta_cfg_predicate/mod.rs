use pyo3::prelude::*;

use crate::types::{AstCfgAttrMeta, AstCfgPredicate};

#[pymethods]
impl AstCfgAttrMeta {
    /// `pub fn cfg_predicate(&self) -> Option<CfgPredicate>`
    fn cfg_predicate(&self) -> Option<AstCfgPredicate> {
        self.inner.cfg_predicate().map(|inner| AstCfgPredicate { inner })
    }
}
