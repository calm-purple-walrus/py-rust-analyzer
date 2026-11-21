pub struct Always;

#[cfg(built)]
pub struct Built;

#[cfg_attr(built, allow(dead_code))]
#[deny(unused_variables)]
pub fn flagged() {}

pub struct Hook {
    pub run: fn(u32) -> String,
}

macro_rules! make {
    () => {
        #[allow(unused)]
        pub struct Made;
    };
}

make!();

// `gen` is a keyword from edition 2024 on; this crate is edition 2021.
#[allow(non_camel_case_types)]
pub struct gen;

pub mod shapes;

#[expect(dead_code)]
fn unused() {}

pub struct Holder<T: Clone + Default>(pub T);

impl Hook {
    /// Runs the hook on zero.
    pub fn start(&self) -> String {
        let Self { run } = self;
        run(0)
    }
}

#[cfg(test)]
#[test]
fn checks() {}

#[cfg(test)]
#[test]
#[ignore]
fn later() {}

/// A field of each form of type.
pub struct Forms<'a, T> {
    pub array: [u8; 4],
    pub pointer: *mut u16,
    pub slice: &'a [u32],
    pub pair: (u8, i8),
    pub param: T,
    pub shape: &'a dyn shapes::Shape,
    pub result: Result<u8, String>,
    pub range: std::ops::Range<u32>,
}
