use crate::iter::Iter;
use crate::types::{AstMatchArm, AstMatchArmList};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchArmList {
    /// `fn arms(&self) -> AstChildren<MatchArm>`
    fn arms(&self) -> Iter {
        Iter::new(
            self.inner.arms(),
            |py, inner| Ok(Py::new(py, AstMatchArm { inner })?.into_any()),
            None,
        )
    }
}
