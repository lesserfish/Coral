use crate::coral::cartridge::mapper::types::*;
use crate::coral::cartridge::types;


#[derive(Copy, Clone, Debug)] 
pub struct Mapper0 {
    pub prg_banks : u8,
    pub chr_banks : u8
}

impl MapperT for Mapper0 {
    fn cpu_r_mapt(&mut self, address : u16) -> usize {
        let uaddress = address as usize;
        if self.prg_banks > 1 { uaddress & 0x7FFF } else { uaddress & 0x3FFF }
    }
    fn cpu_w_mapt(&mut self, address : u16, _byte : u8) -> Option<usize> {
        let uaddress = address as usize;
        if self.prg_banks > 1 { Some(uaddress & 0x7FFF) } else { Some(uaddress & 0x3FFF) }
    }
    fn ppu_r_mapt(&mut self, address : u16) -> usize {
        address as usize
    }
    fn ppu_w_mapt(&mut self, address : u16, _byte : u8) -> Option<usize> {
        Some(address as usize)
    }
    fn clone_self(&self) -> Box<dyn MapperT> {
        Box::new(self.clone())
    }
    fn reset(&mut self){}
}


pub fn new(prg_banks : u8, chr_banks : u8) -> Mapper0 {
    Mapper0 { prg_banks, chr_banks }
}

pub fn choose(cartridge : &mut types::Cartridge){
    let prg_banks = cartridge.header.h_prg_size;
    let chr_banks = cartridge.header.h_chr_size;
    let mapper0 = new(prg_banks, chr_banks);
    cartridge.mapper = Mapper{0: Box::new(mapper0)}
}
