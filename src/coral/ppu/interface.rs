use crate::ppu::types::*;
use crate::ppu::primitive::*;
use crate::utils;

// CPU Read API
fn read_status<T : Bus>(bus : &mut T) -> u8 {
   let data_buffer = get_data_buffer(bus); 
   let status = get_status(bus);
   let byte = (status & 0xE0) | (data_buffer & 0x1F);
   set_write_toggle(bus, false);
   set_status_flag(bus, StatusFlag::VerticalBlank, false);
   byte
}
fn read_data<T : Bus>(bus : &mut T) -> u8 {
    let vram = get_vram(bus);
    let old_data_buffer = get_data_buffer(bus);
    let new_data_buffer = bus.read_byte(vram);
    set_data_buffer(bus, new_data_buffer);
    let output = if vram >= 0x3F00 { new_data_buffer } else { old_data_buffer };

    let increment_mode = get_control_flag(bus, ControlFlag::IncrementMode);
    if increment_mode {
        set_vram(bus, vram + 32);
    } else {
        set_vram(bus, vram + 1);
    }
    output
}
fn read_oam_data<T : Bus>(bus : &mut T) -> u8 {
    let address = get_oam_address(bus);
    bus.get_ppu().oam_data[address as usize]
}

pub fn cpu_read<T : Bus>(bus : &mut T, address : u16) -> u8 {
    match address {
        0x0000 => { 0 }
        0x0001 => { 0 }
        0x0002 => { read_status(bus) }
        0x0003 => { 0 }
        0x0004 => { read_oam_data(bus) }
        0x0005 => { 0 }
        0x0006 => { 0 }
        0x0007 => { read_data(bus) }
        _ => {0}
    }
}

// CPU Peek API
fn peek_status<T : Bus>(bus : &mut T) -> u8 {
   let data_buffer = get_data_buffer(bus); 
   let status = get_status(bus);
   let byte = (status & 0xE0) | (data_buffer & 0x1F);
   byte
}
fn peek_data<T : Bus>(bus : &mut T) -> u8 {
    let vram = get_vram(bus);
    let old_data_buffer = get_data_buffer(bus);
    let new_data_buffer = bus.peek_byte(vram);
    let output = if vram >= 0x3F00 { new_data_buffer } else { old_data_buffer };
    output
}

fn peek_oam_data<T : Bus>(bus : &mut T) -> u8 {
    let address = get_oam_address(bus);
    bus.get_ppu().oam_data[address as usize]
}


pub fn cpu_peek<T : Bus>(bus : &mut T, address : u16) -> u8 {
    match address {
        0x0000 => { 0 }
        0x0001 => { 0 }
        0x0002 => { peek_status(bus) }
        0x0003 => { 0 }
        0x0004 => { peek_oam_data(bus) }
        0x0005 => { 0 }
        0x0006 => { 0 }
        0x0007 => { peek_data(bus) }
        _ => {0}
    }
}

// CPU Write API

fn write_control<T : Bus>(bus : &mut T, byte : u8){
   set_control(bus, byte); 
   let nametable_x = utils::b0(byte);
   let nametable_y = utils::b1(byte);
   set_t_nametable_x(bus, nametable_x);
   set_t_nametable_y(bus, nametable_y);
}

fn write_mask<T : Bus>(bus : &mut T, byte : u8){
    set_mask(bus, byte);
}

fn write_oam_address<T : Bus>(bus : &mut T, byte : u8){
    set_oam_address(bus, byte);
}

fn write_oam_data<T : Bus>(bus : &mut T, byte : u8){
    let oam_address = get_oam_address(bus) as usize;
    bus.fetch_ppu().oam_data[oam_address] = byte;
}

fn write_scroll<T : Bus>(bus : &mut T, byte : u8){
    let write_toggle = get_write_toggle(bus);
    if write_toggle {
        set_write_toggle(bus, false);
        set_t_coarse_x(bus, utils::t5(byte >> 3));
        set_fine_x(bus, utils::t3(byte));
    } else {
        set_write_toggle(bus, true);
        set_t_coarse_y(bus, utils::t5(byte >> 3));
        set_t_fine_y(bus, utils::t3(byte));
    }
}

fn write_address<T : Bus>(bus : &mut T, byte : u8){
    let write_toggle = get_write_toggle(bus);
    if write_toggle {
        set_write_toggle(bus, false);
        let tram = get_tram(bus);
        let new_tram = (byte as u16) | (tram & 0xFF00);
        set_tram(bus, new_tram);
        set_vram(bus, new_tram);

    } else {
        set_write_toggle(bus, true);
        let tram = get_tram(bus);
        let new_tram = (utils::T6(byte as u16) << 8) | (tram & 0x00FF);
        set_tram(bus, new_tram);
    }
}

fn write_data<T : Bus>(bus : &mut T, byte : u8){
    let vram = get_vram(bus);
    bus.write_byte(vram, byte);

    let increment_mode = get_control_flag(bus, ControlFlag::IncrementMode);
    if increment_mode {
        set_vram(bus, vram + 32);
    } else {
        set_vram(bus, vram + 1);
    }
}

pub fn cpu_write<T : Bus>(bus : &mut T, address : u16, byte : u8){
    match address {
        0x0000 => { write_control(bus, byte) }
        0x0001 => { write_mask(bus, byte) }
        0x0002 => { }
        0x0003 => { write_oam_address(bus, byte) }
        0x0004 => { write_oam_data(bus, byte) }
        0x0005 => { write_scroll(bus, byte) }
        0x0006 => { write_address(bus, byte) }
        0x0007 => { write_data(bus, byte) }
        _ => {}
    }
}
