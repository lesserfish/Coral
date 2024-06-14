pub mod types;
pub mod loader;
pub mod mapper;


pub use types::*;
pub use loader::*;
pub use mapper::*;


impl Cartridge {
    pub fn cpu_read(&mut self, address : u16) -> u8{
        let mapped_address = self.mapper.cpu_r_map(address) as usize;
        self.prg_data[mapped_address]
    }
    pub fn cpu_write(&mut self, address : u16, byte : u8){
        let mapped_address = self.mapper.cpu_w_map(address) as usize;
        self.prg_data[mapped_address] = byte;

    }
    pub fn cpu_peek(&mut self, address : u16) -> u8 {
        let mapped_address = self.mapper.cpu_p_map(address) as usize;
        self.prg_data[mapped_address]
    }

    pub fn ppu_read(&mut self, address : u16) -> u8{
        let mapped_address = self.mapper.ppu_r_map(address) as usize;
        self.chr_data[mapped_address]
    }
    pub fn ppu_write(&mut self, address : u16, byte : u8){
        let mapped_address = self.mapper.ppu_w_map(address) as usize;
        self.chr_data[mapped_address] = byte;
    }
    pub fn ppu_peek(&mut self, address : u16) -> u8 {
        let mapped_address = self.mapper.ppu_p_map(address) as usize;
        self.chr_data[mapped_address]
    }

}
