use pyo3::prelude::*;

use crate::types::{LoadCargoConfig, ProcMacroServerChoice};

#[pymethods]
impl LoadCargoConfig {
    /// The struct literal. Rust gives the struct no default, so every field must be given.
    #[new]
    fn new(
        load_out_dirs_from_check: bool,
        with_proc_macro_server: &ProcMacroServerChoice,
        prefill_caches: bool,
        num_worker_threads: usize,
        proc_macro_processes: usize,
    ) -> Self {
        LoadCargoConfig {
            inner: ra_ap_load_cargo::LoadCargoConfig {
                load_out_dirs_from_check,
                with_proc_macro_server: with_proc_macro_server.inner.clone(),
                prefill_caches,
                num_worker_threads,
                proc_macro_processes,
            },
        }
    }
}
