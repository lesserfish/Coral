use crate::coral::cartridge::mapper::types::*;
use crate::coral::cartridge::types;


pub struct Mapper0 {
    pub m0_prg_banks : u8,
    pub m0_chr_banks : u8
}

impl MapperT for Mapper0 {
    fn cpu_r_mapt(&mut self, address : u16) -> u16 {
        if self.m0_prg_banks > 1 { address & 0x7FFF } else { address & 0x3FFF }
    }
    fn cpu_w_mapt(&mut self, address : u16) -> u16 {
        if self.m0_prg_banks > 1 { address & 0x7FFF } else { address & 0x3FFF }
    }
    fn cpu_p_mapt(&self, address : u16) -> u16 {
        if self.m0_prg_banks > 1 { address & 0x7FFF } else { address & 0x3FFF }
    }
    fn ppu_r_mapt(&mut self, address : u16) -> u16 {
        address
    }
    fn ppu_w_mapt(&mut self, address : u16) -> u16 {
        address
    }
    fn ppu_p_mapt(&self, address : u16) -> u16 {
        address
    }
}


pub fn new(prg_banks : u8, chr_banks : u8) -> Mapper0 {
    Mapper0 { m0_prg_banks: prg_banks, m0_chr_banks: chr_banks }
}

pub fn choose(cartridge : &mut types::Cartridge){
    let prg_banks = cartridge.header.h_prg_size;
    let chr_banks = cartridge.header.h_chr_size;
    let mapper0 = new(prg_banks, chr_banks);
    cartridge.mapper = Mapper{0: Box::new(mapper0)}
}
