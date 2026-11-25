use pyo3::prelude::*;

use crate::types::{AssocItem, TypeAlias};

#[pymethods]
impl AssocItem {
    /// `pub fn as_type_alias(self) -> Option<TypeAlias>`
    fn as_type_alias(&self) -> Option<TypeAlias> {
        self.inner.as_type_alias().map(|inner| TypeAlias { inner })
    }
}
