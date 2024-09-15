#![allow(dead_code, unused)]
// it's a rewrite, let's make rustc shut up
// until we are actually somewhat done

use rusqlite::{Connection, Error, OptionalExtension, Result};

use crate::crypto::KeyType;
use crate::ecu_serial::EcuSerial;
use crate::hardware_identifier::HardwareIdentifier;
use crate::public_key::PublicKey;
use crate::secondary_info::SecondaryInfo;
use crate::types::Ecu;

use log::{debug, error};

pub struct SQLStorage {
    conn: Connection,
}

impl SQLStorage {
    pub fn new(database_path: &str) -> Result<Self> {
        let conn = Connection::open(database_path)?;
        Ok(SQLStorage { conn })
    }

    pub fn load_primary_public(&self) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT public FROM primary_keys LIMIT 1;")?;

        match stmt.query_row([], |row| row.get(0)) {
            Ok(public_key) => Ok(Some(public_key)),
            Err(Error::QueryReturnedNoRows) => {
                error!("Uptane public key not found in database");
                Ok(None)
            }
            Err(e) => {
                error!("Failed to get Uptane public key: {}", e);
                Err(e)
            }
        }
    }

    pub fn load_primary_private(&self) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT private FROM primary_keys LIMIT 1;")?;
        match stmt.query_row([], |row| row.get(0)) {
            Ok(private_key) => Ok(Some(private_key)),
            Err(Error::QueryReturnedNoRows) => {
                error!("Uptane private key not found in database");
                Ok(None)
            }
            Err(e) => {
                error!("Failed to get Uptane private key: {}", e);
                Err(e)
            }
        }
    }

    pub fn load_primary_keys(
        &self,
        public_key: &mut String,
        private_key: &mut String,
    ) -> Result<bool> {
        let pub_key = self.load_primary_public()?;
        let priv_key = self.load_primary_private()?;
        if let (Some(pub_key), Some(priv_key)) = (pub_key, priv_key) {
            *public_key = pub_key;
            *private_key = priv_key;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn clear_primary_keys(&self) -> Result<()> {
        self.conn.execute("DELETE FROM primary_keys;", [])?;
        Ok(())
    }

    pub fn load_device_id(&self) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT device_id FROM device_info LIMIT 1;")?;

        let device_id: Option<String> = stmt.query_row([], |row| row.get(0)).optional()?;

        match device_id {
            Some(id) => Ok(Some(id)),
            None => {
                error!("Device ID key not found in database");
                Ok(None)
            }
        }
    }

    pub fn load_tls_credentials(
        &self,
        ca: &mut Vec<u8>,
        cert: &mut Vec<u8>,
        pkey: &mut Vec<u8>,
    ) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT ca_cert, client_cert, client_pkey FROM tls_creds LIMIT 1;")?;

        match stmt.query_row([], |row| {
            let ca_cert: Vec<u8> = row.get(0)?;
            let client_cert: Vec<u8> = row.get(1)?;
            let client_pkey: Vec<u8> = row.get(2)?;
            Ok((ca_cert, client_cert, client_pkey))
        }) {
            Ok((ca_cert, client_cert, client_pkey)) => {
                if ca_cert.is_empty() && client_cert.is_empty() && client_pkey.is_empty() {
                    debug!("All TLS credentials are empty");
                    Ok(false)
                } else {
                    *ca = ca_cert;
                    *cert = client_cert;
                    *pkey = client_pkey;
                    Ok(true)
                }
            }
            Err(Error::QueryReturnedNoRows) => {
                debug!("TLS credentials not found in database");
                Ok(false)
            }
            Err(e) => {
                error!("Failed to get TLS credentials: {}", e);
                Err(e)
            }
        }
    }

    pub fn clear_tls_creds(&self) -> Result<()> {
        self.conn.execute("DELETE FROM tls_creds;", [])?;
        Ok(())
    }

    pub fn load_ecu_registered(&self) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT is_registered FROM device_info LIMIT 1;")?;
        match stmt.query_row([], |row| row.get::<_, i32>(0)) {
            Ok(is_registered) => Ok(is_registered != 0),
            Err(Error::QueryReturnedNoRows) => {
                debug!("Registration flag not found in database");
                Ok(false)
            }
            Err(e) => {
                error!("Failed to get registration flag: {}", e);
                Err(e)
            }
        }
    }

    pub fn load_ecus(&self) -> Result<Vec<Ecu>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, serial, hardware_id, is_primary FROM ecus")?;

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

    pub fn load_secondaries_info(
        &self,
        secondaries: &mut Vec<SecondaryInfo>,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT serial, hardware_id, sec_type, public_key_type, public_key, extra
         FROM ecus
         LEFT JOIN secondary_ecus USING (serial)
         WHERE is_primary = 0
         ORDER BY ecus.id;",
        )?;

        let mut empty = true;

        let rows = stmt.query_map([], |row| {
            let serial_str: String = row.get(0)?;
            let hw_id_str: String = row.get(1)?;
            let sec_type: Option<String> = row.get(2)?;
            let public_key_type: Option<String> = row.get(3)?;
            let public_key_str: Option<String> = row.get(4)?;
            let extra: Option<String> = row.get(5)?;

            let serial = EcuSerial::new(&serial_str).unwrap_or_else(|_| EcuSerial::unknown());
            let hw_id = HardwareIdentifier::new(&hw_id_str)
                .unwrap_or_else(|_| HardwareIdentifier::unknown());

            let pub_key = if let Some(key_type_str) = public_key_type {
                let key_type: KeyType = key_type_str.parse().unwrap_or(KeyType::Unknown);
                if let Some(key_value) = public_key_str {
                    PublicKey::new(&key_value, key_type)
                } else {
                    PublicKey::default()
                }
            } else {
                PublicKey::default()
            };

            let sec_type = sec_type.unwrap_or_else(|| "".to_string());
            let extra = extra.unwrap_or_else(|| "".to_string());

            Ok(SecondaryInfo::new(serial, hw_id, sec_type, pub_key, extra))
        })?;

        for secondary in rows {
            secondaries.push(secondary?);
            empty = false;
        }

        Ok(!empty)
    }
}
