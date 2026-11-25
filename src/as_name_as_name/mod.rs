use pyo3::prelude::*;
use ra_ap_hir_expand::name::AsName;

use crate::types::{AstNameOrNameRef, AstNameRef, Name};

#[pymethods]
impl AstNameOrNameRef {
    /// `AsName::as_name(&self) -> Name`
    fn as_name(&self) -> Name {
        Name { inner: self.inner.as_name() }
    }
}

#[pymethods]
impl AstNameRef {
    /// `AsName::as_name(&self) -> Name`
    fn as_name(&self) -> Name {
        Name { inner: self.inner.as_name() }
    }
}
