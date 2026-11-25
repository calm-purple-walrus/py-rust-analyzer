use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstFieldExpr, EitherFieldTupleField, SemanticsImpl};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn resolve_field(&self, field: &ast::FieldExpr) -> Option<Either<Field, TupleField<'db>>>`
    fn resolve_field(&self, py: Python<'_>, field: &AstFieldExpr) -> Option<EitherFieldTupleField> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.resolve_field(&field.inner))?;
        Some(EitherFieldTupleField { inner, _db: self.db.clone_ref(py) })
    }
}
