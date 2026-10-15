//! Event store event types.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::vclock::VectorClock;

/// Event operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  /// Create operation.
  Create,
  /// Update operation.
  Update,
  /// Delete operation.
  Delete,
}

/// Record type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
  /// Calendar event.
  CalendarEvent,
  /// Todo item.
  Todo,
  /// Note.
  Note,
  /// Contact.
  Contact,
  /// Photo metadata.
  Photo,
}

/// Sync event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
  /// Event ID.
  pub id: Uuid,

  /// Record ID.
  pub record_id: Uuid,

  /// Record type.
  pub record_type: RecordType,

  /// Operation type.
  pub operation: Operation,

  /// Vector clock at event creation.
  pub vclock: VectorClock,

  /// Device ID that created this event.
  pub device_id: String,

  /// Encrypted payload (JSON).
  pub payload: Vec<u8>,

  /// Event timestamp.
  #[serde(with = "time::serde::rfc3339")]
  pub timestamp: OffsetDateTime,
}

impl Event {
  /// Create a new event.
  pub fn new(
    record_id: Uuid,
    record_type: RecordType,
    operation: Operation,
    vclock: VectorClock,
    device_id: String,
    payload: Vec<u8>,
  ) -> Self {
    Self {
      id: Uuid::now_v7(),
      record_id,
      record_type,
      operation,
      vclock,
      device_id,
      payload,
      timestamp: OffsetDateTime::now_utc(),
    }
  }
}