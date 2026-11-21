//! A store that keeps cargo of any kind.

use harbor_domain::Cargo;
use harbor_ports::Store;

use crate::{StorageError, StorageResult};

pub struct Keeper<T> {
    kept: Vec<T>,
}

pub type Archive = Keeper<Cargo>;

impl<T> Keeper<T> {
    pub fn new() -> Keeper<T> {
        Keeper { kept: Vec::new() }
    }
}

impl Store for Archive {
    type Error = StorageError;

    fn save(&mut self, cargo: Cargo) -> StorageResult<()> {
        self.kept.push(cargo);

        Ok(())
    }

    fn count(&self) -> usize {
        self.kept.len()
    }
}
