#[derive(Debug)]
pub struct Ecu {
    pub id: i32,
    pub serial: String,
    pub hardware_id: String,
    pub is_primary: bool,
}
