use crate::held::Held;
use crate::types::{AstRecordExprField, Field, Local, SemanticsImpl, Type};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl SemanticsImpl {
    /// `fn resolve_record_field(&self, field: &ast::RecordExprField) -> Option<(Field, Option<Local<'db>>, Type<'db>)>`
    fn resolve_record_field(
        &self,
        py: Python<'_>,
        field: &AstRecordExprField,
    ) -> Option<(Field, Option<Local>, Type)> {
        let db = self.db.borrow(py);
        let (field, local, ty) =
            attach_db(&db.db, || self.inner.resolve_record_field(&field.inner))?;
        Some((
            Field { inner: field },
            local.map(|inner| Local {
                inner,
                db: self.db.clone_ref(py),
            }),
            Type {
                inner: Held::Own(ty),
                db: self.db.clone_ref(py),
            },
        ))
    }
}
