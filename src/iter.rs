//! Rust iterators for Python.
use pyo3::prelude::*;

/// A Rust iterator, taken one item at a time from Python. It keeps alive the Python object the
/// iterator borrows from, if any.
#[pyclass(unsendable)]
pub struct Iter {
    next: Box<dyn FnMut(Python<'_>) -> PyResult<Option<Py<PyAny>>>>,
    _owner: Option<Py<PyAny>>,
}

impl Iter {
    pub fn new<T: 'static>(
        mut items: impl Iterator<Item = T> + 'static,
        mut convert: impl FnMut(Python<'_>, T) -> PyResult<Py<PyAny>> + 'static,
        owner: Option<Py<PyAny>>,
    ) -> Iter {
        Iter::from_fn(move |py| items.next().map(|item| convert(py, item)).transpose(), owner)
    }

    pub fn from_fn(
        next: impl FnMut(Python<'_>) -> PyResult<Option<Py<PyAny>>> + 'static,
        owner: Option<Py<PyAny>>,
    ) -> Iter {
        Iter { next: Box::new(next), _owner: owner }
    }
}

#[pymethods]
impl Iter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        (self.next)(py)
    }
}
