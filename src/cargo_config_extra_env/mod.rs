use std::collections::HashMap;

use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `pub extra_env: FxHashMap<String, Option<String>>`. Copied, because the setter changes the
    /// `CargoConfig`.
    #[getter]
    fn extra_env(&self) -> HashMap<String, Option<String>> {
        self.inner.extra_env.iter().map(|(key, value)| (key.clone(), value.clone())).collect()
    }

    #[setter]
    fn set_extra_env(&mut self, value: HashMap<String, Option<String>>) {
        self.inner.extra_env = value.into_iter().collect();
    }
}
