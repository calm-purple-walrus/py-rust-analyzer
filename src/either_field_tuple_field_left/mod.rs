use either::Either;
use pyo3::prelude::*;

use crate::types::{EitherFieldTupleField, Field};

#[pymethods]
impl EitherFieldTupleField {
    /// The variant `Either::Left(Field)`, read: its `Field` if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Left")]
    fn as_left(&self) -> Option<Field> {
        match self.inner {
            Either::Left(inner) => Some(Field { inner }),
            Either::Right(_) => None,
        }
    }
}
