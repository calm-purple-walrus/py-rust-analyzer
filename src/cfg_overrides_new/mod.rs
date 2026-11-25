use std::collections::HashMap;

use pyo3::prelude::*;

use crate::types::{CfgDiff, CfgOverrides};

#[pymethods]
impl CfgOverrides {
    /// The struct literal: `global: cfg::CfgDiff` and `selective: FxHashMap<String, cfg::CfgDiff>`. It takes the
    /// diffs, so it is given copies.
    #[new]
    fn new(global: &CfgDiff, selective: HashMap<String, PyRef<'_, CfgDiff>>) -> Self {
        CfgOverrides {
            inner: ra_ap_project_model::CfgOverrides {
                global: global.inner.clone(),
                selective: selective.into_iter().map(|(name, diff)| (name, diff.inner.clone())).collect(),
            },
        }
    }
}
