//! What the tests of the harbor share.

use harbor_domain::{Cargo, CargoAttrs, Port, Weight};

pub fn sample_cargo() -> Cargo {
    let origin = Port { name: String::from("North") };

    Cargo::new(String::from("crates"), Weight::grams(500), origin).expect("a valid cargo")
}

pub fn label_of(cargo: &Cargo) -> String {
    let CargoAttrs { label, .. } = cargo.to_attrs();

    label
}
