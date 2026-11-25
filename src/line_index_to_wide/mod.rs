use pyo3::prelude::*;

use crate::types::{LineCol, LineIndex, WideEncoding, WideLineCol};

#[pymethods]
impl LineIndex {
    /// `pub fn to_wide(&self, enc: WideEncoding, line_col: LineCol) -> Option<WideLineCol>`
    fn to_wide(&self, enc: &WideEncoding, line_col: &LineCol) -> Option<WideLineCol> {
        self.inner.to_wide(enc.inner, line_col.inner).map(|inner| WideLineCol { inner })
    }
}
