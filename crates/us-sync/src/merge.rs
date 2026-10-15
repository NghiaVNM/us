//! CRDT merge logic for conflict resolution.

use std::collections::HashMap;
use uuid::Uuid;

use crate::{
  error::{Error, Result},
  event::{Event, Operation},
};

/// Merger for CRDT conflict resolution.
pub struct Merger;

impl Merger {
  /// Merge events for a record using Last-Write-Wins with Delete-Wins.
  ///
  /// Rules:
  /// 1. Delete always wins over Create/Update
  /// 2. Among Create/Update, use vector clock comparison
  /// 3. If concurrent, use timestamp as tiebreaker
  pub fn merge(events: Vec<Event>) -> Result<Option<Event>> {
    if events.is_empty() {
      return Ok(None);
    }

    // Group by record_id to ensure all events are for same record
    let record_ids: Vec<Uuid> = events.iter().map(|e| e.record_id).collect();
    if !record_ids.windows(2).all(|w| w[0] == w[1]) && record_ids.len() > 1 {
      return Err(Error::ConflictResolution(
        "Events must be for the same record".to_string(),
      ));
    }

    // Delete-wins: if any delete exists, find the latest delete
    let deletes: Vec<&Event> = events
      .iter()
      .filter(|e| e.operation == Operation::Delete)
      .collect();

    if !deletes.is_empty() {
      return Ok(Some(Self::find_winner(&deletes)?.clone()));
    }

    // No deletes, find winner among Create/Update
    let non_deletes: Vec<&Event> = events.iter().collect();
    Ok(Some(Self::find_winner(&non_deletes)?.clone()))
  }

  /// Find the winning event using vector clock comparison and timestamp tiebreaker.
  fn find_winner<'a>(events: &[&'a Event]) -> Result<&'a Event> {
    if events.is_empty() {
      return Err(Error::ConflictResolution("No events to merge".to_string()));
    }

    if events.len() == 1{
      return Ok(events[0]);
    }
    
    let mut winner = events[0];

    for event in events.iter().skip(1) {
      winner = Self::compare_events(winner, event);
    }

    Ok(winner)
  }

  /// Compare two events and return the winner.
  fn compare_events<'a>(a: &'a Event, b: &'a Event) -> &'a Event {
    use std::cmp::Ordering;

    match a.vclock.partial_cmp(&b.vclock) {
      Some(Ordering::Greater) => a,
      Some(Ordering::Less) => b,
      _ => {
        // Concurrent or equal, use timestamp
        if a.timestamp > b.timestamp {
          a
        } else if b.timestamp > a.timestamp {
          b
        } else {
          // Same timestamp, use event ID for determinism
          if a.id > b.id {
            a
          } else {
            b
          }
        }
      }
    }
  }

  /// Merge multiple records' events into a state map.
  pub fn merge_all(events: Vec<Event>) -> Result<HashMap<Uuid, Option<Event>>> {
    let mut by_record: HashMap<Uuid, Vec<Event>> = HashMap::new();

    for event in events {
      by_record.entry(event.record_id).or_default().push(event);
    }

    let mut result = HashMap::new();
    for (record_id, record_events) in by_record {
      result.insert(record_id, Self::merge(record_events)?);
    }

    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{event::RecordType, vclock::VectorClock};

  fn make_event(
    device: &str,
    counter: u64,
    operation: Operation,
    timestamp_offset: i64,
  ) -> Event {
    let mut vclock = VectorClock::new();
    vclock.increment(device);
    for _ in 1..counter {
      vclock.increment(device);
    }

    let mut event = Event::new(
      Uuid::now_v7(),
      RecordType::Note,
      operation,
      vclock,
      device.to_string(),
      vec![],
    );

    event.timestamp = event.timestamp + time::Duration::seconds(timestamp_offset);
    event
  }

  #[test]
  fn test_delete_wins() {
    let record_id = Uuid::now_v7();
    let mut e1 = make_event("device1", 1, Operation::Create, 0);
    let mut e2 = make_event("device2", 1, Operation::Delete, 1);

    e1.record_id = record_id;
    e2.record_id = record_id;

    let result = Merger::merge(vec![e1, e2.clone()]).unwrap();
    assert_eq!(result.unwrap().id, e2.id);
  }

  #[test]
  fn test_vclock_ordering() {
    let record_id = Uuid::now_v7();
    let mut e1 = make_event("device1", 1, Operation::Create, 0);
    let mut e2 = make_event("device1", 2, Operation::Update, 1);

    e1.record_id = record_id;
    e2.record_id = record_id;

    let result = Merger::merge(vec![e1, e2.clone()]).unwrap();
    assert_eq!(result.unwrap().id, e2.id);
  }

  #[test]
  fn test_timestamp_tiebreaker() {
    let record_id = Uuid::now_v7();
    let mut e1 = make_event("device1", 1, Operation::Create, 0);
    let mut e2 = make_event("device2", 1, Operation::Update, 1);

    e1.record_id = record_id;
    e2.record_id = record_id;

    let result = Merger::merge(vec![e1, e2.clone()]).unwrap();
    assert_eq!(result.unwrap().id, e2.id);
  }
}