use super::Cartridge;

pub struct Console {
    pub cartridge: Cartridge,
    pub ram: Vec<i8>,
}
