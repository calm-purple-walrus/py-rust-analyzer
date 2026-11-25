use pyo3::prelude::*;

use crate::types::TextSize;

#[pymethods]
impl TextSize {
    /// `impl From<TextSize> for u32`: `fn from(value: TextSize) -> Self`
    fn __int__(&self) -> u32 {
        u32::from(self.inner)
    }
}
