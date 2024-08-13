use super::mapper;

#[derive(Copy, Clone, Debug, PartialEq)] 
pub enum Mirroring {
    Horizontal,
    Vertical
}

#[derive(Copy, Clone, Debug, PartialEq)] 
pub enum ConsoleType {
    NES,
    NVS,
    Playchoice10,
    Extended,
    Undefined
}

#[derive(Copy, Clone, Debug, PartialEq)] 
pub enum TVSystem {
    NTSC,
    PAL
}

#[derive(Copy, Clone, Debug)] 
pub struct Header {
    pub h_prg_size : u8,
    pub h_chr_size : u8,
    pub h_chr_ram : bool,
    pub h_mirroring : Mirroring,
    pub h_battery : bool,
    pub h_trainer : bool,
    pub h_alt_layout : bool,
    pub h_mapper : u8,
    pub h_console : ConsoleType,
    pub h_nes2 : bool,
    pub h_prg_ram_size : u8,
    pub h_tv_system : TVSystem
}


#[derive(Clone)] 
pub struct Cartridge {
    pub header : Header,
    pub trainer : [u8; 512],
    pub prg_data : Vec<u8>,
    pub chr_data : Vec<u8>,
    pub mapper : mapper::types::Mapper
}

impl Cartridge {
    pub fn cpu_read(&mut self, address : u16) -> u8{
        self.mapper.cpu_read(address)
    }
    pub fn cpu_write(&mut self, address : u16, byte : u8){
        self.mapper.cpu_write(address, byte)
    }
    pub fn ppu_read(&mut self, address : u16) -> u8{
        self.mapper.ppu_read(address)
    }
    pub fn ppu_write(&mut self, address : u16, byte : u8){
        self.mapper.ppu_write(address, byte)
    }
    pub fn reset(&mut self){
        self.mapper.reset()
    }

}
