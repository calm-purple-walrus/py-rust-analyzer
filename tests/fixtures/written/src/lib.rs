//! Code that the tests read as it is written.

pub mod inline {
    pub fn inside() {}
}
pub mod outline;

/// A record.
pub struct Record {
    /// The count.
    pub count: u32,
    pub(crate) shared: u8,
    hidden: bool,
}

pub struct Pair(pub u8, u16);

pub union Bits {
    pub whole: u32,
    pub parts: [u8; 4],
}

pub enum Choice {
    /// The first.
    First,
    Second(u8),
}

pub trait Convert<T> {
    fn convert(&self) -> T;
}

impl Convert<u8> for Pair {
    fn convert(&self) -> u8 {
        self.0
    }
}

impl Record {
    pub fn total(&self) -> Vec<u32> {
        vec![self.count + u32::from(self.shared) + u32::from(self.hidden)]
    }
}

macro_rules! make {
    () => {
        pub struct Made {
            pub field: u8,
        }
    };
}
make!();

#[cfg(feature = "usual")]
pub fn usual() {}

#[cfg(feature = "extra")]
pub fn extra() {}

#[cfg(target_arch = "wasm32")]
pub fn wasm() {}

#[cfg(special)]
pub fn special() {}

#[cfg(flavor = "sweet")]
pub fn sweet() {}

#[cfg_attr(feature = "usual", allow(dead_code, clippy::all))]
#[cfg_attr(feature = "extra", inline)]
pub fn switched() {}

pub fn body(pair: Pair, choice: Choice) -> u8 {
    let kept = pair.convert();
    let Pair(first, _) = pair;
    if let Choice::Second(inner) = choice {
        return inner;
    }
    match first {
        0 => kept,
        _ => first, // ä→ a comment
    }
}
