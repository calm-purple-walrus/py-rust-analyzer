use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::database::Database;
use crate::types::{InFileSmallVecTextRange, InMacroFileSyntaxToken};

#[pymethods]
impl InMacroFileSyntaxToken {
    /// `impl InMacroFile<SyntaxToken>`: `pub fn upmap_once(self, db: &dyn SourceDatabase) -> InFile<SmallVec<[TextRange; 1]>>`.
    /// It takes the value, so it is given a copy.
    fn upmap_once(&self, db: &Database) -> InFileSmallVecTextRange {
        InFileSmallVecTextRange { inner: attach_db(&db.db, || self.inner.clone().upmap_once(&db.db)) }
    }
}
