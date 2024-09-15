use crate::ecu_serial::EcuSerial;
use crate::hardware_identifier::HardwareIdentifier;
use crate::public_key::PublicKey;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SecondaryInfo {
    pub serial: EcuSerial,
    pub hw_id: HardwareIdentifier,
    #[allow(dead_code)]
    pub kind: String,
    #[allow(dead_code)]
    pub pub_key: PublicKey,
    #[allow(dead_code)]
    pub extra: String,
}

impl SecondaryInfo {
    pub fn new(
        serial: EcuSerial,
        hw_id: HardwareIdentifier,
        kind: String,
        pub_key: PublicKey,
        extra: String,
    ) -> Self {
        SecondaryInfo {
            serial,
            hw_id,
            kind,
            pub_key,
            extra,
        }
    }

    pub fn default() -> Self {
        SecondaryInfo {
            serial: EcuSerial::unknown(),
            hw_id: HardwareIdentifier::unknown(),
            kind: String::new(),
            pub_key: PublicKey::default(),
            extra: String::new(),
        }
    }
}

impl fmt::Display for SecondaryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   serial ID: {}", self.serial)?;
        writeln!(f, "   hardware ID: {}", self.hw_id)?;
        writeln!(f, "   no details about installed nor pending images")?;
        writeln!(f, "   public key ID: {}", self.pub_key.key_id())?;
        writeln!(f, "   public key:")?;
        writeln!(f, "{}", self.pub_key)?;
        Ok(())
    }
}
