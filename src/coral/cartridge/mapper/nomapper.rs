use crate::coral::cartridge::mapper::types::*;

pub struct NoMapper {}

impl MapperT for NoMapper {
    fn reset(&mut self) {}
    fn cpu_read(&mut self, _address : u16) -> u8 {0}
    fn cpu_write(&mut self, _address : u16, _byte : u8) {}
    fn ppu_read(&mut self, _address : u16) -> u8 {0}
    fn ppu_write(&mut self, _address : u16, _byte : u8) {}
    fn clone_self(&self) -> Box<dyn MapperT> {
        Box::new(NoMapper{})
    }
}

pub fn new() -> NoMapper {
    NoMapper{}
}

