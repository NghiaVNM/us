//! Vector clock implementation for causality tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Device identifier.
pub type DeviceId = String;

/// Vector clock for tracking causality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
  clock: HashMap<DeviceId, u64>,
}

impl VectorClock {
  /// Create a new empty vector clock.
  pub fn new() -> Self {
    Self {
      clock: HashMap::new(),
    }
  }

  /// Increment the clock for a device.
  pub fn increment(&mut self, device_id: &str) {
    let counter = self.clock.entry(device_id.to_string()).or_insert(0);
    *counter += 1;
  }

  /// Get the counter value for a device.
  pub fn get(&self, device_id: &str) -> u64 {
    self.clock.get(device_id).copied().unwrap_or(0)
  }

  /// Merge two vector clocks (take max of each device).
  pub fn merge(&mut self, other: &VectorClock) {
    for (device_id, &counter) in &other.clock {
      let entry = self.clock.entry(device_id.clone()).or_insert(0);
      *entry = (*entry).max(counter);
    }
  }

  /// Compare two vector clocks for causality.
  pub fn compare(&self, other: &VectorClock) -> Ordering {
    let mut self_greater = false;
    let mut other_greater = false;

    // Check all devices in self
    for (device_id, &self_counter) in &self.clock {
      let other_counter = other.get(device_id);
      if self_counter > other_counter {
        self_greater = true;
      } else if self_counter < other_counter {
        other_greater = true;
      }
    }

    // Check devices only in other
    for (device_id, &other_counter) in &other.clock {
      if !self.clock.contains_key(device_id) && other_counter > 0 {
        other_greater = true;
      }
    }

    match (self_greater, other_greater) {
      (true, false) => Ordering::Greater,
      (false, true) => Ordering::Less,
      (false, false) => Ordering::Equal,
      (true, true) => Ordering::Concurrent,
    }
  }
}

impl PartialOrd for VectorClock {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering;

    let mut self_greater = false;
    let mut other_greater = false;

    // Compare all devices in both clocks
    let all_devices: std::collections::HashSet<_> = self
      .clock
      .keys()
      .chain(other.clock.keys())
      .collect();

    for device in all_devices {
      let self_counter = self.clock.get(device).copied().unwrap_or(0);
      let other_counter = other.clock.get(device).copied().unwrap_or(0);

      match self_counter.cmp(&other_counter) {
        Ordering::Greater => self_greater = true,
        Ordering::Less => other_greater = true,
        Ordering::Equal => {}
      }
    }

    match (self_greater, other_greater) {
      (true, false) => Some(Ordering::Greater),
      (false, true) => Some(Ordering::Less),
      (false, false) => Some(Ordering::Equal),
      (true, true) => None, // Concurrent
    }
  }
}

impl Default for VectorClock {
  fn default() -> Self {
    Self::new()
  }
}

/// Ordering relationship between vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
  /// self > other (self causally after other)
  Greater,
  /// self < other (self causally before other)
  Less,
  /// self == other (identical)
  Equal,
  /// Concurrent (no causal relationship)
  Concurrent,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_increment() {
    let mut vc = VectorClock::new();
    vc.increment("device1");
    vc.increment("device1");
    vc.increment("device2");

    assert_eq!(vc.get("device1"), 2);
    assert_eq!(vc.get("device2"), 1);
    assert_eq!(vc.get("device3"), 0);
  }

  #[test]
  fn test_merge() {
    let mut vc1 = VectorClock::new();
    vc1.increment("device1");
    vc1.increment("device1");

    let mut vc2 = VectorClock::new();
    vc2.increment("device1");
    vc2.increment("device2");

    vc1.merge(&vc2);

    assert_eq!(vc1.get("device1"), 2);
    assert_eq!(vc1.get("device2"), 1);
  }

  #[test]
  fn test_compare_greater() {
    let mut vc1 = VectorClock::new();
    vc1.increment("device1");
    vc1.increment("device1");

    let mut vc2 = VectorClock::new();
    vc2.increment("device1");

    assert_eq!(vc1.compare(&vc2), Ordering::Greater);
  }

  #[test]
  fn test_compare_concurrent() {
    let mut vc1 = VectorClock::new();
    vc1.increment("device1");

    let mut vc2 = VectorClock::new();
    vc2.increment("device2");

    assert_eq!(vc1.compare(&vc2), Ordering::Concurrent);
  }
}