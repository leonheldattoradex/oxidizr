#![allow(dead_code, unused)]
// it's a rewrite, let's make rustc shut up
// until we are actually somewhat done

use crate::ecu_serial::EcuSerial;
use crate::hardware_identifier::HardwareIdentifier;

#[derive(Debug)]
pub struct Ecu {
    pub id: i32,
    pub serial: EcuSerial,
    pub hardware_id: HardwareIdentifier,
    pub is_primary: bool,
}
