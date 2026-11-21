//! Values that rust-analyzer lends instead of handing over.
use std::borrow::Borrow;
use std::ops::Deref;

use pyo3::prelude::*;

/// A value a binding got from rust-analyzer: its own, or lent by the Python object `owner`, which it
/// keeps alive.
pub enum Held<T: ?Sized + ToOwned + 'static> {
    Own(T::Owned),
    Lent { value: &'static T, _owner: Py<PyAny> },
}

impl<T: ?Sized + ToOwned + 'static> Held<T> {
    /// # Safety
    /// `value` must live inside `owner`, and no binding of `owner`'s class may change or move it.
    pub unsafe fn lent(value: &T, owner: Py<PyAny>) -> Self {
        Held::Lent { value: unsafe { extend(value) }, _owner: owner }
    }
}

impl<T: ?Sized + ToOwned + 'static> Deref for Held<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Held::Own(value) => value.borrow(),
            Held::Lent { value, .. } => value,
        }
    }
}

/// # Safety
/// The caller keeps whatever `value` lives in alive and unchanged for as long as the result is used.
pub unsafe fn extend<T: ?Sized>(value: &T) -> &'static T {
    unsafe { &*(value as *const T) }
}

/// The Python object of a binding's receiver, as the owner of what it lends.
pub fn owner<T: pyo3::PyClass>(slf: &Bound<'_, T>) -> Py<PyAny> {
    slf.clone().into_any().unbind()
}
