//! Cargo and what it is made of.

use core::ops::Deref;

use crate::error::DomainError;

pub struct Cargo {
    label: String,
    weight: Weight,
    lines: Vec<Line>,
    origin: Port,
    note: String,
}

impl Cargo {
    pub fn new(label: String, weight: Weight, origin: Port) -> Result<Cargo, DomainError> {
        if label.is_empty() {
            return Err(DomainError::Empty);
        }

        Ok(Cargo { label, weight, lines: Vec::new(), origin, note: String::new() })
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }

    pub fn note(&self) -> &str {
        &self.note
    }

    pub fn to_attrs(&self) -> CargoAttrs {
        CargoAttrs {
            label: self.label.clone(),
            weight: Weight(self.weight.0),
            lines: self.lines.iter().map(Line::to_attrs).collect(),
            origin: Port { name: self.origin.name.clone() },
        }
    }
}

impl Deref for Cargo {
    type Target = Weight;

    fn deref(&self) -> &Weight {
        &self.weight
    }
}

pub struct CargoAttrs {
    pub label: String,
    pub weight: Weight,
    pub lines: Vec<LineAttrs>,
    pub origin: Port,
}

pub struct Weight(u32);

impl Weight {
    pub fn grams(grams: u32) -> Weight {
        Weight(grams)
    }

    pub fn to_attrs(&self) -> WeightAttrs {
        WeightAttrs { grams: self.0 }
    }
}

pub struct WeightAttrs {
    pub grams: u32,
}

pub struct Line {
    sku: String,
    count: u32,
}

impl Line {
    pub fn new(sku: String, count: u32) -> Line {
        Line { sku, count }
    }

    pub fn to_attrs(&self) -> LineAttrs {
        LineAttrs { sku: self.sku.clone(), count: self.count }
    }
}

pub struct LineAttrs {
    pub sku: String,
    pub count: u32,
}

pub enum Mode {
    Sea,
    Air,
    Rail,
}

impl Mode {
    pub fn to_attrs(&self) -> ModeAttrs {
        match self {
            Mode::Sea => ModeAttrs::Sea,
            Mode::Air | Mode::Rail => ModeAttrs::Air,
        }
    }
}

pub enum ModeAttrs {
    Sea,
    Air,
}

pub struct Port {
    pub name: String,
}

impl Port {
    pub fn to_attrs(&self) {}
}

pub struct RouteAttrs {
    pub from: String,
    to: String,
}

impl RouteAttrs {
    pub fn reversed(&self) -> RouteAttrs {
        let RouteAttrs { from, .. } = self;

        RouteAttrs { from: self.to.clone(), to: from.clone() }
    }
}

pub struct ShipCommand {
    pub cargo: Cargo,
    pub to: Port,
}

pub fn parse_weight(text: &str) -> Result<Weight, String> {
    text.parse().map(Weight).map_err(|_| format!("not a weight: {text}"))
}
