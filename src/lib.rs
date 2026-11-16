// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-multifunction-printer-support: multifunction printer support component model.
//!
//! Real implementation: supports, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Support {
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
pub enum MultifunctionPrinterSupportError {
    EmptySupportName,
    DuplicateSupport(String),
    EmptyEventSubject,
}

impl fmt::Display for MultifunctionPrinterSupportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySupportName => write!(f, "support name must not be empty"),
            Self::DuplicateSupport(id) => write!(f, "duplicate support {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MultifunctionPrinterSupportError {}

pub struct MultifunctionPrinterSupport {
    pub name: String,
    supports: BTreeMap<String, Support>,
    events: BTreeMap<String, Event>,
}

impl MultifunctionPrinterSupport {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            supports: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_support(&mut self, item: Support) -> Result<(), MultifunctionPrinterSupportError> {
        if item.name.trim().is_empty() {
            return Err(MultifunctionPrinterSupportError::EmptySupportName);
        }
        let id = item.id.clone();
        if self.supports.contains_key(&id) {
            return Err(MultifunctionPrinterSupportError::DuplicateSupport(id.clone()));
        }
        self.supports.insert(id, item);
        Ok(())
    }

    pub fn support_count(&self) -> usize {
        self.supports.len()
    }

    pub fn get_support(&self, id: &str) -> Option<&Support> {
        self.supports.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MultifunctionPrinterSupportError> {
        if event.subject.trim().is_empty() {
            return Err(MultifunctionPrinterSupportError::EmptyEventSubject);
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

    fn sample_support(id: &str, name: &str) -> Support {
        Support {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MultifunctionPrinterSupport::new("MultifunctionPrinterSupport");
        assert_eq!(m.name, "MultifunctionPrinterSupport");
        assert_eq!(m.support_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_support() {
        let mut m = MultifunctionPrinterSupport::new("MultifunctionPrinterSupport");
        m.add_support(sample_support("a1", "Alpha")).unwrap();
        assert_eq!(m.support_count(), 1);
        assert!(m.get_support("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MultifunctionPrinterSupport::new("MultifunctionPrinterSupport");
        assert!(m.add_support(sample_support("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MultifunctionPrinterSupport::new("MultifunctionPrinterSupport");
        m.add_support(sample_support("x", "One")).unwrap();
        assert!(m.add_support(sample_support("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MultifunctionPrinterSupport::new("MultifunctionPrinterSupport");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
