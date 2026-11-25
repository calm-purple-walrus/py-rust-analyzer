use crate::held::{Held, owner};
use crate::types::{LocalSource, SyntaxNode};
use pyo3::prelude::*;

#[pymethods]
impl LocalSource {
    /// `fn syntax(&self) -> &SyntaxNode`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the syntax node belongs to this immutable LocalSource.
        SyntaxNode {
            inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) },
        }
    }
}
