//! Event store implementation.

use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{error::Result, event::Event, vclock::VectorClock};

/// Event store for CRDT operations.
pub struct EventStore {
  pool: PgPool,
}

impl EventStore {
  /// Create a new event store.
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  /// Append an event to the store.
  pub async fn append(&self, event: &Event) -> Result<()> {
    let vclock_json = serde_json::to_value(&event.vclock)?;

    sqlx::query(
      r#"
      INSERT INTO sync_events (
        id, record_id, record_type, operation,
        vclock, device_id, payload, timestamp
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
      "#,
    )
    .bind(event.id)
    .bind(event.record_id)
    .bind(serde_json::to_string(&event.record_type)?)
    .bind(serde_json::to_string(&event.operation)?)
    .bind(vclock_json)
    .bind(&event.device_id)
    .bind(&event.payload)
    .bind(event.timestamp)
    .execute(&self.pool)
    .await?;

    Ok(())
  }

  /// Get all events for a record.
  pub async fn get_events(&self, record_id: Uuid) -> Result<Vec<Event>> {
    let rows = sqlx::query(
      r#"
      SELECT id, record_id, record_type, operation,
              vclock, device_id, payload, timestamp
      FROM sync_events
      WHERE record_id = $1
      ORDER BY timestamp ASC
      "#,
    )
    .bind(record_id)
    .fetch_all(&self.pool)
    .await?;

    let mut events = Vec::new();
    for row in rows {
      events.push(Event {
        id: row.try_get("id")?,
        record_id: row.try_get("record_id")?,
        record_type: serde_json::from_str(&row.try_get::<String, _>("record_type")?)?,
        operation: serde_json::from_str(&row.try_get::<String, _>("operation")?)?,
        vclock: serde_json::from_value(row.try_get("vclock")?)?,
        device_id: row.try_get("device_id")?,
        payload: row.try_get("payload")?,
        timestamp: row.try_get("timestamp")?,
      });
    }

    Ok(events)
  }

  /// Get vector clock for a device.
  pub async fn get_vclock(&self, device_id: &str) -> Result<VectorClock> {
    let row = sqlx::query(
      r#"
      SELECT vclock
      FROM sync_events
      WHERE device_id = $1
      ORDER BY timestamp DESC
      LIMIT 1
      "#,
    )
    .bind(device_id)
    .fetch_optional(&self.pool)
    .await?;

    match row {
      Some(row) => Ok(serde_json::from_value(row.try_get("vclock")?)?),
      None => Ok(VectorClock::new()),
    }
  }

  /// Get events since a vector clock.
  pub async fn get_events_since(&self, vclock: &VectorClock) -> Result<Vec<Event>> {
    let vclock_json = serde_json::to_value(vclock)?;

    let rows = sqlx::query(
      r#"
      SELECT id, record_id, record_type, operation,
              vclock, device_id, payload, timestamp
      FROM sync_events
      WHERE vclock::jsonb > $1::jsonb
      ORDER BY timestamp ASC
      "#,
    )
    .bind(vclock_json)
    .fetch_all(&self.pool)
    .await?;

    let mut events = Vec::new();
    for row in rows {
      events.push(Event {
        id: row.try_get("id")?,
        record_id: row.try_get("record_id")?,
        record_type: serde_json::from_str(&row.try_get::<String, _>("record_type")?)?,
        operation: serde_json::from_str(&row.try_get::<String, _>("operation")?)?,
        vclock: serde_json::from_value(row.try_get("vclock")?)?,
        device_id: row.try_get("device_id")?,
        payload: row.try_get("payload")?,
        timestamp: row.try_get("timestamp")?,
      });
    }

    Ok(events)
  }
}