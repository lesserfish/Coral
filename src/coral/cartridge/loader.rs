use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

use crate::coral::utils;
use crate::coral::cartridge::types::*;
use crate::coral::cartridge::mapper;

fn new_cartridge() -> Cartridge {
    Cartridge {  header: Header 
                { h_prg_size: 0, 
                  h_chr_size: 0, 
                  h_chr_ram: false, 
                  h_mirroring: Mirroring::Horizontal, 
                  h_battery: false, 
                  h_trainer: false, 
                  h_alt_layout: false, 
                  h_mapper: 0, 
                  h_console: ConsoleType::Undefined, 
                  h_nes2: false, 
                  h_prg_ram_size: 0,
                  h_tv_system: TVSystem::NTSC
                }, 
                trainer: [0;512],
                prg_data: vec![], 
                chr_data: vec![],
                mapper: mapper::generic_mapper()
            }
}

fn get_console_type(byte : u8) -> ConsoleType {
    match byte {
        0 => ConsoleType::NES,
        1 => ConsoleType::NVS,
        2 => ConsoleType::Playchoice10,
        3 => ConsoleType::Extended,
        _ => ConsoleType::Undefined
    }
}

fn process_header(buffer : &mut [u8; 12], cart : &mut Cartridge) -> io::Result<()> {
    let prg_size = buffer[0];
    let chr_size = buffer[1];

    cart.header.h_prg_size = prg_size;
    cart.header.h_chr_size = chr_size;
    cart.header.h_chr_ram = if chr_size == 0 { true } else { false }; 

    // Flag 6
    let flag6 = buffer[2];
    let mirroring = if utils::b0(flag6) {Mirroring::Horizontal} else {Mirroring::Vertical};
    let battery = utils::b1(flag6);
    let trainer = utils::b2(flag6);
    let alt_layout = utils::b3(flag6);
    let mapper_lsn = flag6 >> 4;

    cart.header.h_mirroring = mirroring;
    cart.header.h_battery = battery;
    cart.header.h_trainer = trainer;
    cart.header.h_alt_layout = alt_layout;
    cart.header.h_mapper = mapper_lsn;

    // Flag 7
    let flag7 = buffer[3];
    let console_type = get_console_type(flag7 & 0x3);
    let nes2 = utils::b3(flag7);
    let mapper_msn = flag7 & 0xF0;

    cart.header.h_console = console_type;
    cart.header.h_nes2 = nes2;
    cart.header.h_mapper += mapper_msn;

    // Flag 8
    let prg_ram_size = buffer[4];
    cart.header.h_prg_ram_size = prg_ram_size;

    // Flag 9
    let flag9 = buffer[5];
    let tv_system = if utils::b0(flag9) {TVSystem::NTSC} else {TVSystem::PAL};

    cart.header.h_tv_system = tv_system;


    // Flag 10
    // Not part of the official speficiation. Few emulators honor this. We are no better.
    
    // Remaining 5 bits: Padding. Set to 0
    Ok(())
}

fn load_header(file : &mut File, cart : &mut Cartridge) -> io::Result<()> {
    let mut magic_numbers : [u8; 4] = [0; 4];
    file.read_exact(&mut magic_numbers)?;

    if magic_numbers != [0x4E, 0x45, 0x53, 0x1A] {
        return Err(Error::new(ErrorKind::InvalidData, "Failed to parse: Missing magic numbers. File is not a valid .NES file."));
    }

    let mut header_buffer : [u8; 12] = [0; 12];
    file.read_exact(&mut header_buffer)?;

    process_header(&mut header_buffer, cart)?;

    Ok(())
}


// TODO: Implement this
fn load_trainer(file : &mut File, cart : &mut Cartridge) -> io::Result<()> { 
    let has_trainer = cart.header.h_trainer;

    if has_trainer {
        file.read_exact(&mut cart.trainer)?;
    }
    Ok(())
} 

fn load_prg(file : &mut File, cart : &mut Cartridge) -> io::Result<()> {
    let buffer_size = 0x4000 * (cart.header.h_prg_size as usize);
    cart.prg_data.resize(buffer_size, 0);
    file.read_exact(&mut cart.prg_data)?;
    Ok(())
}

fn load_chr(file : &mut File, cart : &mut Cartridge) -> io::Result<()> {
    let chr_size = if cart.header.h_chr_ram { 1 } else { cart.header.h_chr_size } as usize;
    let chr_data_size = 0x2000 * chr_size;
    cart.chr_data.resize(chr_data_size, 0);

    let buffer_size = 0x2000 * cart.header.h_chr_size as usize;
    file.read_exact(&mut cart.chr_data[..buffer_size])?;

    Ok(())
}

// TODO: Implement this
fn load_playchoice(_file : &mut File, _cart : &mut Cartridge) -> io::Result<()> {
    Ok(())
}

fn setup_mapper(cart : &mut Cartridge) -> io::Result<()> {
    mapper::choose_mapper(cart)?;
    Ok(())
}

pub fn load<T : AsRef<Path>>(filepath : T) -> io::Result<Cartridge> {
    let mut cart = new_cartridge();
    let mut file = File::open(filepath)?;
    load_header(&mut file, &mut cart)?;
    load_trainer(&mut file, &mut cart)?;
    load_prg(&mut file, &mut cart)?;
    load_chr(&mut file, &mut cart)?;
    load_playchoice(&mut file, &mut cart)?;
    setup_mapper(&mut cart)?;
    Ok(cart)
}
