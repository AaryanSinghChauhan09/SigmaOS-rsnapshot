// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rsnapshot: rsnapshot component model.
//!
//! Real implementation: rsnapshot_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsnapshotItem {
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
pub enum RsnapshotError {
    EmptyRsnapshotItemName,
    DuplicateRsnapshotItem(String),
    EmptyEventSubject,
}

impl fmt::Display for RsnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRsnapshotItemName => write!(f, "rsnapshot_item name must not be empty"),
            Self::DuplicateRsnapshotItem(id) => write!(f, "duplicate rsnapshot_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RsnapshotError {}

pub struct Rsnapshot {
    pub name: String,
    rsnapshot_items: BTreeMap<String, RsnapshotItem>,
    events: BTreeMap<String, Event>,
}

impl Rsnapshot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rsnapshot_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_rsnapshot_item(&mut self, item: RsnapshotItem) -> Result<(), RsnapshotError> {
        if item.name.trim().is_empty() {
            return Err(RsnapshotError::EmptyRsnapshotItemName);
        }
        let id = item.id.clone();
        if self.rsnapshot_items.contains_key(&id) {
            return Err(RsnapshotError::DuplicateRsnapshotItem(id.clone()));
        }
        self.rsnapshot_items.insert(id, item);
        Ok(())
    }

    pub fn rsnapshot_item_count(&self) -> usize {
        self.rsnapshot_items.len()
    }

    pub fn get_rsnapshot_item(&self, id: &str) -> Option<&RsnapshotItem> {
        self.rsnapshot_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RsnapshotError> {
        if event.subject.trim().is_empty() {
            return Err(RsnapshotError::EmptyEventSubject);
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

    fn sample_rsnapshot_item(id: &str, name: &str) -> RsnapshotItem {
        RsnapshotItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Rsnapshot::new("Rsnapshot");
        assert_eq!(m.name, "Rsnapshot");
        assert_eq!(m.rsnapshot_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_rsnapshot_item() {
        let mut m = Rsnapshot::new("Rsnapshot");
        m.add_rsnapshot_item(sample_rsnapshot_item("a1", "Alpha")).unwrap();
        assert_eq!(m.rsnapshot_item_count(), 1);
        assert!(m.get_rsnapshot_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Rsnapshot::new("Rsnapshot");
        assert!(m.add_rsnapshot_item(sample_rsnapshot_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Rsnapshot::new("Rsnapshot");
        m.add_rsnapshot_item(sample_rsnapshot_item("x", "One")).unwrap();
        assert!(m.add_rsnapshot_item(sample_rsnapshot_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Rsnapshot::new("Rsnapshot");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
