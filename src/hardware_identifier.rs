#![allow(dead_code, unused)]
// it's a rewrite, let's make rustc shut up
// until we are actually somewhat done

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use std::cmp::Ordering;
use std::fmt;

const HWID_MAX_LENGTH: usize = 200;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HardwareIdentifier(String);

impl HardwareIdentifier {
    pub fn new(hwid: &str) -> Result<Self, HardwareIdentifierError> {
        if hwid.len() > HWID_MAX_LENGTH {
            return Err(HardwareIdentifierError(format!(
                "Hardware Identifier is too long (max {} characters)",
                HWID_MAX_LENGTH
            )));
        }
        Ok(HardwareIdentifier(hwid.to_string()))
    }

    pub fn unknown() -> Self {
        HardwareIdentifier("Unknown".to_string())
    }

    pub fn to_string(&self) -> &str {
        &self.0
    }
}

impl FromSql for HardwareIdentifier {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(hwid_str) => match HardwareIdentifier::new(hwid_str) {
                Ok(hwid) => Ok(hwid),
                Err(e) => Err(FromSqlError::Other(Box::new(e))),
            },
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl fmt::Display for HardwareIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ord for HardwareIdentifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for HardwareIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct HardwareIdentifierError(String);

impl fmt::Display for HardwareIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HardwareIdentifier Error: {}", self.0)
    }
}

impl std::error::Error for HardwareIdentifierError {}
