pub mod types;
mod mapper0;

use std::io;
use std::io::Error;
use std::io::ErrorKind;

use crate::coral::cartridge as Cartridge;

pub fn choose_mapper(cartridge : &mut Cartridge::Cartridge) -> io::Result<()>{
    match cartridge.header.h_mapper {
        0 => { 
            mapper0::choose(cartridge);
            Ok(())
        }
        _ => {
            Err(Error::new(ErrorKind::Other, "Unsupported mapper. My bad :("))
        }
    }
}

pub fn generic_mapper() -> types::Mapper {
    types::Mapper{0 : Box::new(mapper0::new(0, 0))}
}

impl types::Mapper {
    pub fn cpu_r_map(&mut self, address : u16) -> u16{
        self.0.cpu_r_mapt(address)
    }
    pub fn cpu_w_map(&mut self, address : u16) -> u16{
        self.0.cpu_w_mapt(address)
    }
    pub fn cpu_p_map(&mut self, address : u16) -> u16{
        self.0.cpu_p_mapt(address)
    }
    pub fn ppu_r_map(&mut self, address : u16) -> u16{
        self.0.ppu_r_mapt(address)
    }
    pub fn ppu_w_map(&mut self, address : u16) -> u16{
        self.0.ppu_w_mapt(address)
    }
    pub fn ppu_p_map(&mut self, address : u16) -> u16{
        self.0.ppu_p_mapt(address)
    }
}
