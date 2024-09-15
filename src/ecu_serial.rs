use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use std::cmp::Ordering;
use std::fmt;

const ECU_SERIAL_MIN_LENGTH: usize = 1;
const ECU_SERIAL_MAX_LENGTH: usize = 64;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EcuSerial(String);

impl EcuSerial {
    pub fn new(serial: &str) -> Result<Self, EcuSerialError> {
        if serial.len() < ECU_SERIAL_MIN_LENGTH {
            return Err(EcuSerialError(format!(
                "ECU serial identifier is too short (min {} characters)",
                ECU_SERIAL_MIN_LENGTH
            )));
        }
        if serial.len() > ECU_SERIAL_MAX_LENGTH {
            return Err(EcuSerialError(format!(
                "ECU serial identifier is too long (max {} characters)",
                ECU_SERIAL_MAX_LENGTH
            )));
        }
        Ok(EcuSerial(serial.to_string()))
    }
    pub fn unknown() -> Self {
        EcuSerial("Unknown".to_string())
    }
}

impl fmt::Display for EcuSerial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ord for EcuSerial {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for EcuSerial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromSql for EcuSerial {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(serial_str) => match EcuSerial::new(serial_str) {
                Ok(ecu_serial) => Ok(ecu_serial),
                Err(e) => Err(FromSqlError::Other(Box::new(e))),
            },
            Err(_e) => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug)]
pub struct EcuSerialError(String);

impl fmt::Display for EcuSerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EcuSerial Error: {}", self.0)
    }
}

impl std::error::Error for EcuSerialError {}
