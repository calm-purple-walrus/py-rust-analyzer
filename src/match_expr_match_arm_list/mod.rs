use crate::types::{AstMatchArmList, AstMatchExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchExpr {
    /// `fn match_arm_list(&self) -> Option<MatchArmList>`
    fn match_arm_list(&self) -> Option<AstMatchArmList> {
        self.inner
            .match_arm_list()
            .map(|inner| AstMatchArmList { inner })
    }
}
