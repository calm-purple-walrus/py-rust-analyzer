use crate::held::{extend, owner};
use crate::types::{
    InFileAstConst, InFileAstName, InFileAstStatic, InFileAstTypeAlias, InFileEitherAstMacroAstFn,
    InFileEitherAstTypeOrConstParamAstTrait, InFileSyntaxNodeRef,
};
use pyo3::prelude::*;

#[pymethods]
impl InFileAstConst {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}

#[pymethods]
impl InFileAstStatic {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}

#[pymethods]
impl InFileAstTypeAlias {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}

#[pymethods]
impl InFileEitherAstMacroAstFn {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}

#[pymethods]
impl InFileEitherAstTypeOrConstParamAstTrait {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}

#[pymethods]
impl InFileAstName {
    /// `fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> InFileSyntaxNodeRef {
        let this = slf.borrow();
        // SAFETY: the original AST node belongs to this immutable InFile, which the result holds.
        let node = this.inner.syntax().map(|syntax| unsafe { extend(syntax) });
        InFileSyntaxNodeRef {
            inner: node,
            _owner: owner(slf),
        }
    }
}
