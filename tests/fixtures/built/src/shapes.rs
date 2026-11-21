//! Shapes of code that the wrapper's tests read.

extern crate alloc;

use std::fmt;

/// A record.
pub struct Record {
    /// The count.
    pub count: u32,

    hidden: bool,
}

pub struct Pair(pub u8, u16);

pub struct Unit;

pub enum Choice {
    /// The first.
    First,
    Second(u8),
    Third { inner: u8 },
}

pub mod nested {
    pub fn inside() {}
}

pub trait Shape {
    fn area(&self) -> u32;

    fn label(&self) -> &str {
        "shape"
    }
}

impl Record {
    pub fn new(count: u32) -> Self {
        Self { count, hidden: false }
    }
}

impl Shape for Record {
    fn area(&self) -> u32 {
        self.count
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unit")
    }
}

// A plain comment.
#[allow(dead_code)]
#[cfg_attr(built, deny(unused_variables))]
#[doc = "Flows."]
#[must_use]
pub fn flows(values: &[u32]) -> Option<u32> {
    let first = *values.first()?;
    let Record { count, .. } = Record::new(first);
    let Record { count: _, hidden } = Record::new(count);
    for value in values {
        if *value > 3 {
            return Some(*value);
        }
    }
    while hidden {}
    loop {
        break;
    }
    match first {
        0 => {}
        _ => {}
    }
    if let Some(found) = values.last() {
        (found + 1).checked_add(0)?;
    }
    let total = Record::new(1).area();
    let read = Record::new(2).count;
    let _ = read;
    {
        let _inner = (read, &first);
    }
    Some(total)
}

pub async fn waits() -> u32 {
    ready().await
}

async fn ready() -> u32 {
    1
}
