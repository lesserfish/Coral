pub mod types;
mod mapper0;

use std::fmt::format;
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
            let error_message = format!("Mapper {} is not yet supported. My bad :(", cartridge.header.h_mapper);
            Err(Error::new(ErrorKind::Other, error_message))
        }
    }
}

pub fn generic_mapper() -> types::Mapper {
    types::Mapper{0 : Box::new(mapper0::new(0, 0))}
}


