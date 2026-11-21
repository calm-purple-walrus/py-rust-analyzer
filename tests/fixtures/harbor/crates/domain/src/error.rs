//! What can go wrong in the model.

#[derive(Debug)]
pub enum DomainError {
    Empty,
    TooHeavy { grams: u32 },
}
