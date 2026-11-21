//! A store that keeps cargo in a list.

use harbor_domain::Cargo;
use harbor_ports::{ApplicationError, Store};

use crate::StorageError;

pub struct MemoryStore {
    kept: Vec<Cargo>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        MemoryStore { kept: Vec::new() }
    }

    pub fn first_label(&self) -> Result<String, StorageError> {
        let cargo = self.kept.first().ok_or(StorageError)?;

        Ok(cargo.to_attrs().label)
    }

    pub fn require_any(&self) -> Result<(), ApplicationError> {
        if self.kept.is_empty() {
            return Err(ApplicationError::Missing);
        }

        Ok(())
    }
}

impl Store for MemoryStore {
    type Error = ApplicationError;

    fn save(&mut self, cargo: Cargo) -> Result<(), ApplicationError> {
        self.kept.push(cargo);

        Ok(())
    }

    fn count(&self) -> usize {
        self.kept.len()
    }
}

#[cfg(test)]
mod tests {
    use harbor_ports::Store;
    use harbor_testing::sample_cargo;

    use super::MemoryStore;

    #[test]
    fn keeps_what_it_is_given() {
        let mut store = MemoryStore::new();

        store.save(sample_cargo()).expect("saved");
        assert_eq!(store.count(), 1);
    }
}
