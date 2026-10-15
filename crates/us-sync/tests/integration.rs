//! Integration tests for sync services.

use us_sync::{
  error::Result,
  event::{Event, Operation, RecordType},
  merge::Merger,
  vclock::VectorClock,
};
use uuid::Uuid;

#[test]
fn test_event_creation() -> Result<()> {
  let record_id = Uuid::now_v7();
  let mut vclock = VectorClock::new();
  vclock.increment("device1");

  let event = Event::new(
    record_id,
    RecordType::Note,
    Operation::Create,
    vclock.clone(),
    "device1".to_string(),
    vec![1, 2, 3],
  );

  assert_eq!(event.record_id, record_id);
  assert_eq!(event.record_type, RecordType::Note);
  assert_eq!(event.operation, Operation::Create);
  assert_eq!(event.device_id, "device1");
  assert_eq!(event.payload, vec![1, 2, 3]);

  Ok(())
}

#[test]
fn test_vclock_causality() {
  let mut vc1 = VectorClock::new();
  vc1.increment("device1");
  vc1.increment("device1");

  let mut vc2 = VectorClock::new();
  vc2.increment("device1");

  // vc1 > vc2
  assert!(vc1.partial_cmp(&vc2) == Some(std::cmp::Ordering::Greater));
  assert!(vc2.partial_cmp(&vc1) == Some(std::cmp::Ordering::Less));
}

#[test]
fn test_vclock_concurrent() {
  let mut vc1 = VectorClock::new();
  vc1.increment("device1");

  let mut vc2 = VectorClock::new();
  vc2.increment("device2");

  // vc1 and vc2 are concurrent
  assert!(vc1.partial_cmp(&vc2).is_none());
  assert!(vc2.partial_cmp(&vc1).is_none());
}

#[test]
fn test_merge_delete_wins() -> Result<()> {
  let record_id = Uuid::now_v7();

  let mut vc1 = VectorClock::new();
  vc1.increment("device1");

  let mut e1 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Create,
    vc1,
    "device1".to_string(),
    vec![],
  );
  e1.record_id = record_id;

  let mut vc2 = VectorClock::new();
  vc2.increment("device2");

  let mut e2 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Delete,
    vc2,
    "device2".to_string(),
    vec![],
  );
  e2.record_id = record_id;

  let result = Merger::merge(vec![e1, e2.clone()])?;
  assert!(result.is_some());
  assert_eq!(result.unwrap().operation, Operation::Delete);

  Ok(())
}

#[test]
fn test_merge_vclock_ordering() -> Result<()> {
  let record_id = Uuid::now_v7();

  let mut vc1 = VectorClock::new();
  vc1.increment("device1");

  let mut e1 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Create,
    vc1,
    "device1".to_string(),
    vec![],
  );
  e1.record_id = record_id;

  let mut vc2 = VectorClock::new();
  vc2.increment("device1");
  vc2.increment("device1");

  let mut e2 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Create,
    vc2,
    "device1".to_string(),
    vec![],
  );
  e2.record_id = record_id;

  let result = Merger::merge(vec![e1, e2.clone()])?;
  assert!(result.is_some());
  assert_eq!(result.unwrap().id, e2.id);

  Ok(())
}

#[test]
fn test_merge_concurrent_uses_timestamp() -> Result<()> {
  let record_id = Uuid::now_v7();

  let mut vc1 = VectorClock::new();
  vc1.increment("device1");

  let mut e1 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Create,
    vc1,
    "device1".to_string(),
    vec![],
  );
  e1.record_id = record_id;

  // Sleep to ensure different timestamp
  let mut vc2 = VectorClock::new();
  vc2.increment("device2");

  let mut e2 = Event::new(
    record_id,
    RecordType::Note,
    Operation::Update,
    vc2,
    "device2".to_string(),
    vec![],
  );
  e2.record_id = record_id;

  let result = Merger::merge(vec![e1, e2.clone()])?;
  assert!(result.is_some());
  // e2 should win due to later timestamp
  assert_eq!(result.unwrap().id, e2.id);

  Ok(())
}

#[test]
fn test_merge_empty() -> Result<()> {
  let result = Merger::merge(vec![])?;
  assert!(result.is_none());
  Ok(())
}