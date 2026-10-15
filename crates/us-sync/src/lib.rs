//! Sync service core - event store and CRDT merge logic.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod event;
pub mod merge;
pub mod store;
pub mod vclock;

pub use error::{Error, Result};
pub use event::{Event, Operation, RecordType};
pub use merge::Merger;
pub use store::EventStore;
pub use vclock::VectorClock;