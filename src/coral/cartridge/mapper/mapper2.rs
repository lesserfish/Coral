use crate::coral::cartridge::mapper::types::*;
use crate::coral::cartridge::types;


#[derive(Copy, Clone, Debug)] 
pub struct Mapper2 {
    selected_bank : usize,
    switchable_banks : u8
}

impl MapperT for Mapper2 {
    fn cpu_r_mapt(&mut self, address : u16) -> usize {
        let uaddress = address as usize;
        if address >= 0x8000 && address <= 0xBFFF {
            let bank_address = self.selected_bank * 0x4000;
            let bank_offset = uaddress & 0x3FFF;
            bank_address + bank_offset
        } else {
            let bank_address = (self.switchable_banks as usize - 1) * 0x4000;
            let bank_offset = uaddress & 0x3FFF;
            bank_address + bank_offset
        }
    }
    fn cpu_w_mapt(&mut self, address : u16, byte : u8) -> Option<usize> {
        if address >= 0x8000 {
            self.selected_bank = (byte & 0x7) as usize;
            None
        } else {
            Some(address as usize)
        }
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


pub fn new(switchable_banks : u8) -> Mapper2 {
    Mapper2 { selected_bank: 0, switchable_banks }
}

pub fn choose(cartridge : &mut types::Cartridge){
    let mapper2 = new(cartridge.header.h_prg_size);
    cartridge.mapper = Mapper{0: Box::new(mapper2)}
}
