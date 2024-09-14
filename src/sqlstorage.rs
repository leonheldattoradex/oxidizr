use rusqlite::{Connection, Result, OptionalExtension};

use crate::types::Ecu;

pub struct SQLStorage {
    conn: Connection,
}

impl SQLStorage {
    pub fn new(database_path: &str) -> Result<Self> {
        let conn = Connection::open(database_path)?;
        Ok(SQLStorage { conn })
    }

    pub fn load_device_id(&self) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT device_id FROM device_info LIMIT 1;")?;
        
        let device_id: Option<String> = stmt.query_row([], |row| row.get(0)).optional()?;
        
        match device_id {
            Some(id) => Ok(Some(id)),
            None => {
                println!("Device ID key not found in database");
                Ok(None)
            }
        }
    }

    pub fn load_ecus(&self) -> Result<Vec<Ecu>> {
        let mut stmt = self.conn.prepare("SELECT id, serial, hardware_id, is_primary FROM ecus")?;
        
        let ecu_iter = stmt.query_map([], |row| {
            Ok(Ecu {
                id: row.get(0)?,
                serial: row.get(1)?,
                hardware_id: row.get(2)?,
                is_primary: row.get::<_, i32>(3)? != 0,
            })
        })?;

        let ecus = ecu_iter.collect::<Result<Vec<Ecu>, _>>()?;
        
        Ok(ecus)
    }
}
