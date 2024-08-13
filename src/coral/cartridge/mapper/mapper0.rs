use crate::coral::cartridge::mapper::types::*;
use crate::coral::cartridge::types;


#[derive(Clone, Debug)] 
pub struct Mapper0 {
    pub prg_banks : usize,
    pub chr_banks : usize,
    prg_data : Vec<u8>,
    chr_data : Vec<u8>,
}

impl Mapper0 {
    fn cpu_r_map(&mut self, address : u16) -> usize {
        let uaddress = address as usize;
        if self.prg_banks > 1 { uaddress & 0x7FFF } else { uaddress & 0x3FFF }
    }
    fn cpu_w_map(&mut self, address : u16, _byte : u8) -> Option<usize> {
        let uaddress = address as usize;
        if self.prg_banks > 1 { Some(uaddress & 0x7FFF) } else { Some(uaddress & 0x3FFF) }
    }
    fn ppu_r_map(&mut self, address : u16) -> usize {
        address as usize
    }
    fn ppu_w_map(&mut self, address : u16, _byte : u8) -> Option<usize> {
        Some(address as usize)
    }
}


impl MapperT for Mapper0 {
    fn cpu_read(&mut self, address : u16) -> u8 {
        let mapped_address = self.cpu_r_map(address);
        self.prg_data[mapped_address]
    }
    fn cpu_write(&mut self, address : u16, byte : u8) {
        let optional_map = self.cpu_w_map(address, byte);
        match optional_map {
            Some(mapped_address) => { self.prg_data[mapped_address] = byte; }
            None => {}
        }
    }
    fn ppu_read(&mut self, address : u16) -> u8 {
        let mapped_address = self.ppu_r_map(address);
        self.chr_data[mapped_address]
    }
    fn ppu_write(&mut self, address : u16, byte : u8) {
        let optional_map = self.ppu_w_map(address, byte);
        match optional_map {
            Some(mapped_address) => { self.chr_data[mapped_address] = byte; }
            None => {}
        }
    }
    fn reset(&mut self) {
        // TODO: Not implemented. Fix.
    }
    fn clone_self(&self) -> Box<dyn MapperT> {
        Box::new(self.clone())
    }
}


pub fn choose(cartridge : &mut types::Cartridge){

    let prg_banks = cartridge.header.h_prg_size as usize;
    let chr_banks = cartridge.header.h_chr_size as usize;

    let prg_data_size = 0x4000 * prg_banks;
    let chr_data_size = if chr_banks == 0 {0x2000} else {0x2000 * chr_banks};

    let mut prg_data = vec![0; prg_data_size];
    let mut chr_data = vec![0; chr_data_size];


    prg_data[..cartridge.prg_data.len()].copy_from_slice(&cartridge.prg_data);
    chr_data[..cartridge.chr_data.len()].copy_from_slice(&cartridge.chr_data);

    let mapper0 = Mapper0 { prg_banks, chr_banks, prg_data, chr_data};
    cartridge.mapper = Mapper{0: Box::new(mapper0)}
}
