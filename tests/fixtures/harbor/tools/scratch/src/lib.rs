//! Code that has not found its layer yet.

mod layout;

pub use layout::{Bin, Plain, Signal, after_imports, chosen, early, ends_in_if, flows, gathered, held, returned, right_below, spaced};

pub struct Report {
    pub north: u32,
    pub south: u32,
    pub east: u32,
    pub west: u32,
    pub inbound: u32,
    pub outbound: u32,
    pub stored: u32,
    pub damaged: u32,
    pub lost: u32,
    pub found: u32,
    pub weighed: u32,
    pub counted: u32,
    pub sealed: u32,
    pub opened: u32,
    pub loaded: u32,
    pub unloaded: u32,
}

#[allow(clippy::too_many_lines)]
pub fn ledger(start: u32) -> u32 {
    let a1 = start + 1;
    let a2 = a1 + 1;
    let a3 = a2 + 1;
    let a4 = a3 + 1;
    let a5 = a4 + 1;
    let a6 = a5 + 1;
    let a7 = a6 + 1;
    let a8 = a7 + 1;
    let a9 = a8 + 1;
    let a10 = a9 + 1;
    let a11 = a10 + 1;
    let a12 = a11 + 1;

    let b1 = a12 + 1;
    let b2 = b1 + 1;
    let b3 = b2 + 1;
    let b4 = b3 + 1;
    let b5 = b4 + 1;
    let b6 = b5 + 1;
    let b7 = b6 + 1;
    let b8 = b7 + 1;
    let b9 = b8 + 1;
    let b10 = b9 + 1;
    let b11 = b10 + 1;

    let c1 = b11 + 1;
    let c2 = c1 + 1;
    let c3 = c2 + 1;
    let c4 = c3 + 1;
    let c5 = c4 + 1;
    let c6 = c5 + 1;
    let c7 = c6 + 1;
    let c8 = c7 + 1;
    let c9 = c8 + 1;
    let c10 = c9 + 1;

    let d1 = c10 + 1;
    let d2 = d1 + 1;
    let d3 = d2 + 1;
    let d4 = d3 + 1;
    let d5 = d4 + 1;
    let d6 = d5 + 1;
    let d7 = d6 + 1;
    let d8 = d7 + 1;
    let d9 = d8 + 1;
    let d10 = d9 + 1;

    d10
}

pub fn grade(score: u32) -> u32 {
    match score {
        value if value < 10 => 1,
        value if value < 20 => 2,
        value if value < 30 => 3,
        value if value < 40 => 4,
        value if value < 50 => 5,
        value if value < 60 => 6,
        value if value < 70 => 7,
        value if value < 80 => 8,
        value if value < 90 => 9,
        value if value < 100 => 10,
        value if value < 110 => 11,
        value if value < 120 => 12,
        value if value < 130 => 13,
        value if value < 140 => 14,
        value if value < 150 => 15,
        _ => 16,
    }
}

pub fn tally(rows: &[Vec<u32>], out: &mut Vec<u32>) {
    for row in rows {
        for value in row {
            if value.is_power_of_two() {
                if row.is_sorted() {
                    if out.is_sorted() {
                        out.push(*value);
                    }
                }
            }
        }
    }
}
