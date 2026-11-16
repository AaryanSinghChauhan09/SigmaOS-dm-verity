// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dm-verity: dm verity component model.
//!
//! Real implementation: veritys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verity {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmVerityError {
    EmptyVerityName,
    DuplicateVerity(String),
    EmptyEventSubject,
}

impl fmt::Display for DmVerityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVerityName => write!(f, "verity name must not be empty"),
            Self::DuplicateVerity(id) => write!(f, "duplicate verity {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DmVerityError {}

pub struct DmVerity {
    pub name: String,
    veritys: BTreeMap<String, Verity>,
    events: BTreeMap<String, Event>,
}

impl DmVerity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            veritys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_verity(&mut self, item: Verity) -> Result<(), DmVerityError> {
        if item.name.trim().is_empty() {
            return Err(DmVerityError::EmptyVerityName);
        }
        let id = item.id.clone();
        if self.veritys.contains_key(&id) {
            return Err(DmVerityError::DuplicateVerity(id.clone()));
        }
        self.veritys.insert(id, item);
        Ok(())
    }

    pub fn verity_count(&self) -> usize {
        self.veritys.len()
    }

    pub fn get_verity(&self, id: &str) -> Option<&Verity> {
        self.veritys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DmVerityError> {
        if event.subject.trim().is_empty() {
            return Err(DmVerityError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_verity(id: &str, name: &str) -> Verity {
        Verity {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DmVerity::new("DmVerity");
        assert_eq!(m.name, "DmVerity");
        assert_eq!(m.verity_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_verity() {
        let mut m = DmVerity::new("DmVerity");
        m.add_verity(sample_verity("a1", "Alpha")).unwrap();
        assert_eq!(m.verity_count(), 1);
        assert!(m.get_verity("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DmVerity::new("DmVerity");
        assert!(m.add_verity(sample_verity("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DmVerity::new("DmVerity");
        m.add_verity(sample_verity("x", "One")).unwrap();
        assert!(m.add_verity(sample_verity("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DmVerity::new("DmVerity");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
