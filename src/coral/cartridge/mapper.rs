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


