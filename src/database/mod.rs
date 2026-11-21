use pyo3::prelude::*;
use ra_ap_ide_db::RootDatabase;

/// The database every call of `hir` reads from. It is never changed after it is made, so the
/// values that borrow from it stay valid while their Python objects hold it.
/// `ide_db::RootDatabase`
#[pyclass(unsendable, name = "RootDatabase")]
pub struct Database {
    pub db: RootDatabase,
}
