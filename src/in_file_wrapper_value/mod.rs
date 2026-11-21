use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstAdt, AstFn, AstModule, InFileAstAdt, InFileAstModule, InFileFn, InFileSmallVecTextRange, InFileSyntaxNode, InFileTextRange, SyntaxNode, TextRange};

// `pub value: T` of `InFileWrapper<FileKind, T>`, for each `T` Python uses.

#[pymethods]
impl InFileAstAdt {
    /// `pub value: ast::Adt`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> AstAdt {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        AstAdt { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InFileFn {
    /// `pub value: ast::Fn`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> AstFn {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        AstFn { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InFileTextRange {
    /// `pub value: TextRange`
    #[getter]
    fn value(&self) -> TextRange {
        TextRange { inner: self.inner.value }
    }
}

#[pymethods]
impl InFileSyntaxNode {
    /// `pub value: SyntaxNode`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        SyntaxNode { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InFileSmallVecTextRange {
    /// `pub value: SmallVec<[TextRange; 1]>`, as a list.
    #[getter]
    fn value(&self) -> Vec<TextRange> {
        self.inner.value.iter().map(|&inner| TextRange { inner }).collect()
    }
}

#[pymethods]
impl InFileAstModule {
    /// `pub value: T`, with `T` the `ast::Module`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> AstModule {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        AstModule { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}
