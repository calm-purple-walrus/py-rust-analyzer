//! The cargo as it travels between harbors.

use harbor_domain::{CargoAttrs, LineAttrs, ModeAttrs, Port, Weight};

pub struct WireCargo {
    pub label: String,
    pub weight: Weight,
    pub lines: Option<WireLine>,
    pub origin: Port,
}

pub struct WireLine {
    pub sku: String,
    pub count: u32,
}

pub enum WireMode {
    Sea,
    Air,
    Road,
}

#[derive(Debug)]
pub struct WireError;

impl From<CargoAttrs> for WireCargo {
    fn from(CargoAttrs { label, weight, lines, origin }: CargoAttrs) -> Self {
        WireCargo { label, weight, lines: lines.into_iter().next().map(WireLine::from), origin }
    }
}

impl From<LineAttrs> for WireLine {
    fn from(LineAttrs { sku, .. }: LineAttrs) -> Self {
        WireLine { sku, count: 1 }
    }
}

impl TryFrom<WireMode> for ModeAttrs {
    type Error = WireError;

    fn try_from(wire: WireMode) -> Result<Self, Self::Error> {
        match wire {
            WireMode::Sea => Ok(ModeAttrs::Sea),
            WireMode::Air => Ok(ModeAttrs::Air),
            WireMode::Road => Err(WireError),
        }
    }
}
