//! What the application needs from outside.

use harbor_domain::{Cargo, DomainError};

#[derive(Debug)]
pub enum ApplicationError {
    Missing,
    Refused(DomainError),
}

pub trait Store {
    type Error;

    fn save(&mut self, cargo: Cargo) -> Result<(), Self::Error>;

    fn count(&self) -> usize;
}

pub fn refused(error: DomainError) -> Result<(), ApplicationError> {
    Err(ApplicationError::Refused(error))
}
