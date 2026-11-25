use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{
    Adt, AstAdt, AstConst, AstFn, AstImpl, AstStatic, AstTrait, AstTypeAlias, Const, Function, Impl, SemanticsImpl,
    Static, Trait, TypeAlias,
};

/// The nodes `to_def` is bound for.
#[derive(FromPyObject)]
pub enum Source<'py> {
    Fn(PyRef<'py, AstFn>),
    Static(PyRef<'py, AstStatic>),
    Const(PyRef<'py, AstConst>),
    TypeAlias(PyRef<'py, AstTypeAlias>),
    Adt(PyRef<'py, AstAdt>),
    Impl(PyRef<'py, AstImpl>),
    Trait(PyRef<'py, AstTrait>),
}

#[pymethods]
impl SemanticsImpl {
    /// `pub fn to_def<T: ToDef<'db>>(&self, src: &T) -> Option<T::Def>`
    fn to_def(&self, py: Python<'_>, src: Source<'_>) -> PyResult<Option<Py<PyAny>>> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || match src {
            Source::TypeAlias(src) => self.inner.to_def(&src.inner).map(|inner| TypeAlias { inner }.into_py_any(py)),
            Source::Const(src) => self.inner.to_def(&src.inner).map(|inner| Const { inner }.into_py_any(py)),
            Source::Static(src) => self.inner.to_def(&src.inner).map(|inner| Static { inner }.into_py_any(py)),
            Source::Fn(src) => self.inner.to_def(&*src.inner).map(|inner| Function { inner }.into_py_any(py)),
            Source::Adt(src) => self.inner.to_def(&*src.inner).map(|inner| Adt { inner }.into_py_any(py)),
            Source::Impl(src) => self.inner.to_def(&*src.inner).map(|inner| Impl { inner }.into_py_any(py)),
            Source::Trait(src) => self.inner.to_def(&*src.inner).map(|inner| Trait { inner }.into_py_any(py)),
        })
        .transpose()
    }
}
