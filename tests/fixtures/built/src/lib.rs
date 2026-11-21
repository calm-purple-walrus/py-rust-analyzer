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
