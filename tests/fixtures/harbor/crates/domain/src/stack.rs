//! A stack that keeps its label beside its items.

use core::fmt::{Debug, Display};

pub struct Stack<T: Clone + Debug, U: Display>
where
    T: Default,
{
    items: Vec<T>,
    label: U,
}

impl<T: Clone + Debug + Default, U: Display> Stack<T, U> {
    pub fn new(label: U) -> Self {
        Stack { items: Vec::new(), label }
    }

    pub fn top(&self) -> T {
        self.items.last().cloned().unwrap_or_default()
    }

    pub fn label(&self) -> String {
        self.label.to_string()
    }
}
