use std::vec::Vec;

pub struct Cartridge {
    /// PRG-ROM banks
    pub prg: Vec<u8>,
    /// CHR-ROM banks
    pub chr: Vec<u8>,
    /// Save RAM
    pub s_ram: Vec<u8>,
    /// mapper type
    pub mapper: u8,
    /// Mirroring mode
    pub mirror: u8,
    /// battery present
    pub battery: u8,
}

impl Cartridge {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, mapper: u8, mirror: u8, battery: u8) -> Self {
        Cartridge {
            prg,
            chr,
            s_ram: vec![0; 0x2000],
            mapper,
            mirror,
            battery,
        }
    }

    #[allow(dead_code)]
    fn save(&self) {
        // TODO
    }

    #[allow(dead_code)]
    fn load(&self) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_constructor() {
        let c = Cartridge::new(Vec::new(), Vec::new(), 0, 0, 0);

        assert_eq!(8192, c.s_ram.len());
    }
}
