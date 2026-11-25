use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{EditionedFileId, SyntaxError};

#[pymethods]
impl EditionedFileId {
    /// `pub fn parse_errors(self, db: &dyn SourceDatabase) -> Option<&[SyntaxError]>`, which the query returns
    /// by reference: the errors as a list of copies.
    fn parse_errors(&self, db: &Database) -> Option<Vec<SyntaxError>> {
        attach_db(&db.db, || self.inner.parse_errors(&db.db))
            .map(|errors| errors.iter().map(|inner| SyntaxError { inner: inner.clone() }).collect())
    }
}
