use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::held::Held;
use crate::types::{AstRecordPat, Field, SemanticsImpl, Type};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn record_pattern_missing_fields(&self, pattern: &ast::RecordPat) -> Vec<(Field, Type<'db>)>`
    fn record_pattern_missing_fields(&self, py: Python<'_>, pattern: &AstRecordPat) -> Vec<(Field, Type)> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.record_pattern_missing_fields(&pattern.inner))
            .into_iter()
            .map(|(field, ty)| (Field { inner: field }, Type { inner: Held::Own(ty), db: self.db.clone_ref(py) }))
            .collect()
    }
}
