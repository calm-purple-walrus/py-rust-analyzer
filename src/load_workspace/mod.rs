use std::collections::HashMap;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::database::Database;
use crate::types::{LoadCargoConfig, ProcMacroClient, ProjectWorkspace, Vfs};

/// `pub fn load_workspace(ws: ProjectWorkspace, extra_env: &FxHashMap<String, Option<String>>, load_config: &LoadCargoConfig) -> anyhow::Result<(RootDatabase, Vfs, Option<ProcMacroClient>)>`
#[pyfunction]
pub fn load_workspace(
    ws: &ProjectWorkspace,
    extra_env: HashMap<String, Option<String>>,
    load_config: &LoadCargoConfig,
) -> PyResult<(Database, Vfs, Option<ProcMacroClient>)> {
    let ws = ws.inner.clone();
    let extra_env = extra_env.into_iter().collect();
    let (db, vfs, client) = ra_ap_load_cargo::load_workspace(ws, &extra_env, &load_config.inner)
        .map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))?;
    Ok((Database { db }, Vfs { inner: vfs }, client.map(|inner| ProcMacroClient { inner })))
}
