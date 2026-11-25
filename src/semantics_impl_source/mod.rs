use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{
    Adt, Const, EnumVariant, Field, Function, Impl, InFileAstAdt, InFileAstConst, InFileAstImpl, InFileAstStatic,
    InFileAstTrait, InFileAstTypeAlias, InFileAstVariant, InFileEitherAstMacroAstFn,
    InFileEitherAstTypeOrConstParamAstTrait, InFileFieldSource, InFileFn, Macro, SemanticsImpl, Static, Trait,
    TypeAlias, TypeOrConstParam,
};

/// The definitions `source` is bound for.
#[derive(FromPyObject)]
pub enum Def<'py> {
    Function(PyRef<'py, Function>),
    Const(PyRef<'py, Const>),
    Static(PyRef<'py, Static>),
    TypeAlias(PyRef<'py, TypeAlias>),
    Macro(PyRef<'py, Macro>),
    TypeOrConstParam(PyRef<'py, TypeOrConstParam>),
    Adt(PyRef<'py, Adt>),
    Impl(PyRef<'py, Impl>),
    Trait(PyRef<'py, Trait>),
    Field(PyRef<'py, Field>),
    EnumVariant(PyRef<'py, EnumVariant>),
}

#[pymethods]
impl SemanticsImpl {
    /// `pub fn source<Def: HasSource>(&self, def: Def) -> Option<InFile<Def::Ast>>`
    fn source(&self, py: Python<'_>, def: Def<'_>) -> PyResult<Option<Py<PyAny>>> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || match def {
            Def::Const(def) => self.inner.source(def.inner).map(|inner| InFileAstConst { inner }.into_py_any(py)),
            Def::Static(def) => self.inner.source(def.inner).map(|inner| InFileAstStatic { inner }.into_py_any(py)),
            Def::TypeAlias(def) => self.inner.source(def.inner).map(|inner| InFileAstTypeAlias { inner }.into_py_any(py)),
            Def::Macro(def) => self.inner.source(def.inner).map(|inner| InFileEitherAstMacroAstFn { inner }.into_py_any(py)),
            Def::TypeOrConstParam(def) => {
                self.inner.source(def.inner).map(|inner| InFileEitherAstTypeOrConstParamAstTrait { inner }.into_py_any(py))
            }
            Def::Function(def) => self.inner.source(def.inner).map(|inner| InFileFn { inner }.into_py_any(py)),
            Def::Adt(def) => self.inner.source(def.inner).map(|inner| InFileAstAdt { inner }.into_py_any(py)),
            Def::Impl(def) => self.inner.source(def.inner).map(|inner| InFileAstImpl { inner }.into_py_any(py)),
            Def::Trait(def) => self.inner.source(def.inner).map(|inner| InFileAstTrait { inner }.into_py_any(py)),
            Def::Field(def) => self.inner.source(def.inner).map(|inner| InFileFieldSource { inner }.into_py_any(py)),
            Def::EnumVariant(def) => {
                self.inner.source(def.inner).map(|inner| InFileAstVariant { inner }.into_py_any(py))
            }
        })
        .transpose()
    }
}
