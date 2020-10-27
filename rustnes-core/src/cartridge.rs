pub struct Cartridge {
    /// PRG-ROM banks
    pub PRG: [i8],
    /// CHR-ROM banks
    pub CHR: [i8],
    /// Save RAM
    pub SRAM: [i8],
    /// mapper type
    pub Mapper: i8,
    /// Mirroring mode
    pub Mirror: i8,
    /// battery present
    pub Battery: i8,
}
