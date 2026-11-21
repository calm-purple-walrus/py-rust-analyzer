use pyo3::prelude::*;

use crate::types::DepKind;

#[pymethods]
impl DepKind {
    /// `impl Debug for DepKind`. The variants cannot be read, because the type cannot be named.
    ///
    /// Python tells the kinds apart by this text (`repr(kind) == "Dev"`). That works because `DepKind` derives
    /// `Debug`, so the text is the variant's name: `Normal`, `Dev` or `Build`. If a rust-analyzer upgrade renamed a
    /// variant, every comparison with the old name would be false, and dev-dependencies would count as normal ones.
    /// `test_a_package_lends_its_dependencies` reads all three kinds and would fail then.
    fn __repr__(&self) -> String {
        format!("{:?}", self.dependency.kind)
    }
}
