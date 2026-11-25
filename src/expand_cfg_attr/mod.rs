use std::ops::ControlFlow;

use pyo3::prelude::*;

use crate::types::{AstAttr, AstMeta, CfgOptions};

/// `pub fn expand_cfg_attr<'a, BreakValue>(attrs: impl Iterator<Item = ast::Attr>, cfg_options: impl FnMut() -> &'a CfgOptions, callback: impl FnMut(ast::Meta, ast::Attr) -> ControlFlow<BreakValue>) -> Option<BreakValue>`
///
/// `attrs` is a list, and `cfg_options` the options themselves. `callback` is a Python callable that takes a
/// `Meta` and an `Attr`: what it returns breaks, and is returned, unless it is `None`, which continues. If it
/// raises, the binding panics.
#[pyfunction]
pub fn expand_cfg_attr(
    py: Python<'_>,
    attrs: Vec<PyRef<'_, AstAttr>>,
    cfg_options: &CfgOptions,
    callback: Py<PyAny>,
) -> Option<Py<PyAny>> {
    ra_ap_hir_expand::attrs::expand_cfg_attr(
        attrs.iter().map(|attr| attr.inner.clone()),
        || &cfg_options.inner,
        |meta, attr| match callback.call1(py, (AstMeta { inner: meta }, AstAttr { inner: attr })) {
            Ok(value) if value.is_none(py) => ControlFlow::Continue(()),
            Ok(value) => ControlFlow::Break(value),
            Err(error) => panic!("the expand_cfg_attr callback raised {error}"),
        },
    )
}
