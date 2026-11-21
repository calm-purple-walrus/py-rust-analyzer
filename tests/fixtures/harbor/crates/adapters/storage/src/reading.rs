//! Reads cargo through its attributes.

use harbor_domain::{Cargo, CargoAttrs, Line, LineAttrs, WeightAttrs};

pub fn describe(cargo: &Cargo) -> String {
    let CargoAttrs { label, weight, lines, origin } = cargo.to_attrs();
    let WeightAttrs { grams } = weight.to_attrs();

    format!("{label}: {grams} grams in {} lines from {}", lines.len(), origin.name)
}

pub fn label_of(cargo: &Cargo) -> String {
    let CargoAttrs { label, .. } = cargo.to_attrs();

    label
}

pub fn sku(line: &Line) -> String {
    let LineAttrs { sku, count: _ } = line.to_attrs();

    sku
}

pub fn labels(cargo: &[Cargo]) -> Vec<String> {
    let mut labels = Vec::new();

    for each in cargo {
        let attrs = each.to_attrs();

        labels.push(attrs.label);
    }

    labels
}

pub fn weighed(cargo: &Cargo) -> usize {
    counted(cargo.to_attrs())
}

fn counted(attrs: CargoAttrs) -> usize {
    attrs.lines.len()
}

pub fn span(range: core::ops::Range<u32>) -> u32 {
    let core::ops::Range { start, .. } = range;

    start
}

pub fn first_sku(cargo: &Cargo) -> String {
    let LineAttrs { sku, .. } = cargo.to_attrs().lines.remove(0);

    sku
}
