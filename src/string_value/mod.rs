use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::types::AstString;

#[pymethods]
impl AstString {
    /// `fn value(&self) -> Result<Cow<'_, str>, EscapeError>`; an invalid literal raises ValueError.
    fn value(&self) -> PyResult<String> {
        self.inner.value().map(|text| text.into_owned()).map_err(|error| PyValueError::new_err(format!("{error:?}")))
    }
}
