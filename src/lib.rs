//! SigmaOS-accountability: accountability model.
//!
//! Real implementation: responsibility, ownership, and validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Responsibility {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub deadline: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountabilityError {
    EmptyName,
    DuplicateResponsibility(String),
}

impl fmt::Display for AccountabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "responsibility name must not be empty"),
            Self::DuplicateResponsibility(id) => write!(f, "duplicate responsibility {:?}", id),
        }
    }
}

impl std::error::Error for AccountabilityError {}

pub struct AccountabilitySystem {
    pub name: String,
    responsibilities: BTreeMap<String, Responsibility>,
}

impl AccountabilitySystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), responsibilities: BTreeMap::new() }
    }

    pub fn add_responsibility(&mut self, r: Responsibility) -> Result<(), AccountabilityError> {
        if r.name.trim().is_empty() { return Err(AccountabilityError::EmptyName); }
        let id = r.id.clone();
        if self.responsibilities.contains_key(&id) {
            return Err(AccountabilityError::DuplicateResponsibility(id.clone()));
        }
        self.responsibilities.insert(id, r);
        Ok(())
    }

    pub fn responsibility_count(&self) -> usize { self.responsibilities.len() }

    pub fn get_responsibility(&self, id: &str) -> Option<&Responsibility> { self.responsibilities.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let s = AccountabilitySystem::new("My System");
        assert_eq!(s.name, "My System");
        assert_eq!(s.responsibility_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut s = AccountabilitySystem::new("My System");
        let r = Responsibility { id: "r1".into(), name: "bugfix".into(), owner: "dev".into(), deadline: "2024-01-01".into() };
        s.add_responsibility(r).unwrap();
        assert_eq!(s.responsibility_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut s = AccountabilitySystem::new("My System");
        let r = Responsibility { id: "x".into(), name: "".into(), owner: "a".into(), deadline: "b".into() };
        assert!(s.add_responsibility(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = AccountabilitySystem::new("My System");
        let r = Responsibility { id: "x".into(), name: "X".into(), owner: "a".into(), deadline: "b".into() };
        s.add_responsibility(r).unwrap();
        let r2 = Responsibility { id: "x".into(), name: "X".into(), owner: "a".into(), deadline: "b".into() };
        assert!(s.add_responsibility(r2).is_err());
    }
}
