//! Code laid out with and without blank lines.

use std::collections::BTreeMap;
use std::fmt::Write;
pub fn after_imports() -> BTreeMap<u32, String> {
    BTreeMap::new()
}
pub fn right_below() -> String {
    let mut text = String::new();

    writeln!(text, "one").expect("written");

    text
}

pub fn spaced() -> u32 {
    3
}

pub fn flows(values: &[u32]) -> u32 {
    let mut total = 0;
    for value in values {
        total += value;
    }
    let doubled = total * 2;

    if doubled > 10 {
        return doubled;
    }

    total
}

pub fn chosen(flag: bool) -> u32 {
    let size = if flag { 1 } else { 2 };
    let more = size + 1;

    more
}

pub fn gathered() -> Vec<usize> {
    let values = vec![
        1,
        2,
    ];
    let more = values.len();

    vec![more]
}

pub fn returned(value: u32) -> u32 {
    let doubled = value * 2;
    doubled + 1
}

pub fn ends_in_if(value: u32) -> u32 {
    let half = value / 2;
    if half > 1 { half } else { 1 }
}

/// A bin on a shelf.
pub struct Bin {
    /// What is in it.
    pub content: String,
    pub weight: u32,

    /// Where it stands.
    pub place: u32,
}

impl Bin {
    pub fn new(content: String) -> Bin {
        Bin { content, weight: 0, place: 0 }
    }
    pub fn heavy(&self) -> bool {
        self.weight > 10
    }
}

pub struct Plain {
    pub first: u32,
    pub second: u32,
}

pub enum Signal {
    /// Nothing to do.
    Idle,
    Busy {
        /// How long it takes.
        seconds: u32,
        load: u32,
    },
}

// This is just, basically, the sum of what the bins hold.
pub fn held(bins: &[Bin]) -> u32 {
    bins.iter().map(|bin| bin.weight).sum()
}

#[cfg(test)]
mod tests {
    use super::flows;

    #[test]
    fn counts_rows() {
        let mut rows = 0;
        for _ in 0..3 {
            rows += 1;
        }
        assert_eq!(rows, 3);
    }

    #[test]
    fn flows_add_up() {
        // The values obviously add up.
        assert_eq!(flows(&[1, 2]), 3);
    }
}

pub fn early(value: u32) -> u32 {
    let half = value / 2;
    return half;
}
