use pyo3::prelude::*;

use crate::types::TextSize;

#[pymethods]
impl TextSize {
    /// `impl Sub<TextSize> for TextSize`: `fn sub(self, rhs: TextSize) -> TextSize`
    #[pyo3(name = "sub_TextSize")]
    fn sub(&self, rhs: &TextSize) -> TextSize {
        TextSize { inner: self.inner - rhs.inner }
    }
}
