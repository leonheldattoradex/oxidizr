#![allow(dead_code, unused)]
// it's a rewrite, let's make rustc shut up
// until we are actually somewhat done

use crate::crypto::{Crypto, KeyType};
use serde_json::Value;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PublicKey {
    value: String,
    key_type: KeyType,
}

impl PublicKey {
    pub fn new(value: &str, key_type: KeyType) -> Self {
        PublicKey {
            value: value.to_string(),
            key_type,
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let value = fs::read_to_string(path)?;
        let key_type = Crypto::identify_rsa_key_type(&value)?;
        Ok(PublicKey { value, key_type })
    }

    pub fn from_json(uptane_json: &Value) -> Result<Self, Box<dyn Error>> {
        let keytype = uptane_json["keytype"]
            .as_str()
            .ok_or("Invalid key type")?
            .to_lowercase();
        let keyvalue = uptane_json["keyval"]["public"]
            .as_str()
            .ok_or("Invalid key value")?
            .to_string();

        let key_type = match keytype.as_str() {
            "ed25519" => KeyType::Ed25519,
            "rsa" => Crypto::identify_rsa_key_type(&keyvalue)?,
            _ => KeyType::Unknown,
        };

        Ok(PublicKey {
            value: keyvalue,
            key_type,
        })
    }

    pub fn verify_signature(&self, signature: &str, message: &str) -> bool {
        match self.key_type {
            KeyType::Ed25519 => Crypto::ed25519_verify(&self.value, signature, message),
            KeyType::Rsa2048 | KeyType::Rsa3072 | KeyType::Rsa4096 => {
                Crypto::rsa_pss_verify(&self.value, signature, message)
            }
            _ => false,
        }
    }

    pub fn to_uptane(&self) -> Value {
        let mut res = serde_json::json!({});
        match self.key_type {
            KeyType::Rsa2048 | KeyType::Rsa3072 | KeyType::Rsa4096 => {
                res["keytype"] = Value::String("RSA".to_string());
            }
            KeyType::Ed25519 => {
                res["keytype"] = Value::String("ED25519".to_string());
            }
            KeyType::Unknown => {
                res["keytype"] = Value::String("unknown".to_string());
            }
        }
        res["keyval"]["public"] = Value::String(self.value.clone());
        res
    }

    pub fn key_id(&self) -> String {
        Crypto::sha256digest_hex(&self.value)
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey {
            value: "Unknown".to_string(),
            key_type: KeyType::Unknown,
        }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key_lines: Vec<&str> = self.value.lines().collect();
        for line in key_lines {
            writeln!(f, "   {}", line)?;
        }
        Ok(())
    }
}
