use pyo3::prelude::*;

use crate::held::{Held, extend, owner};
use crate::types::{
    InRealFileSyntaxNode,
    AstAdt, AstFn, AstImpl, AstModule, AstTrait, AstVariant, FieldSource, InFileAstAdt, InFileAstImpl,
    InFileAstModule, InFileAstTrait, InFileAstVariant, InFileFieldSource, InFileFn, InFileSmallVecTextRange,
    InFileSyntaxNode, InFileSyntaxNodeRef, InFileTextRange, SyntaxNode, TextRange,
};

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

#[pymethods]
impl InFileAstImpl {
    /// `pub value: T`, with `T` the `ast::Impl`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> AstImpl {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        AstImpl { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InFileAstTrait {
    /// `pub value: T`, with `T` the `ast::Trait`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> AstTrait {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        AstTrait { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InFileFieldSource {
    /// `pub value: T`, with `T` the `FieldSource`, lent by this `InFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> FieldSource {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes an `InFile`.
        FieldSource { inner: unsafe { extend(&this.inner.value) }, _owner: owner(slf) }
    }
}

#[pymethods]
impl InFileAstVariant {
    /// `pub value: T`, with `T` the `ast::Variant`, as a copy.
    #[getter]
    fn value(&self) -> AstVariant {
        AstVariant { inner: self.inner.value.clone() }
    }
}

#[pymethods]
impl InFileSyntaxNodeRef {
    /// `pub value: &SyntaxNode`
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the immutable InFile holds the owner of its syntax node.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.value, owner(slf)) } }
    }
}

#[pymethods]
impl InRealFileSyntaxNode {
    /// `pub value: SyntaxNode`, lent by this `InRealFile`.
    #[getter]
    fn value(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node belongs to this immutable InRealFile.
        SyntaxNode { inner: unsafe { Held::lent(&this.inner.value, owner(slf)) } }
    }
}
