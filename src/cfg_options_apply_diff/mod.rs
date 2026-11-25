use pyo3::prelude::*;

use crate::types::{CfgDiff, CfgOptions};

#[pymethods]
impl CfgOptions {
    /// `pub fn apply_diff(&mut self, diff: CfgDiff)`. It takes the diff, so it is given a copy.
    fn apply_diff(&mut self, diff: &CfgDiff) {
        self.inner.apply_diff(diff.inner.clone());
    }
}
