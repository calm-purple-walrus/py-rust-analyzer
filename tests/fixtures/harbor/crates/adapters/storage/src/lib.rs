//! Keeps cargo in memory.

mod archive;
mod memory;
mod reading;

pub use archive::{Archive, Keeper};
pub use memory::MemoryStore;
pub use reading::{describe, first_sku, label_of, labels, sku, span, weighed};

#[derive(Debug)]
pub struct StorageError;

pub type StorageResult<T> = Result<T, StorageError>;
