#![allow(unused)]

use crate::{mos::types::Bus, mos::primitive::*, mos::types::AddrMode, mos::types::Flag};
use crate::utils::{self, join_bytes, page_cross_sum, split_bytes};

// Addressing Modes

fn get_address<T : Bus>(bus : &mut T, address_mode : AddrMode) -> u16 {
    return match address_mode {
        AddrMode::Immediate => ga_immediate(bus),
        AddrMode::Zeropage  => ga_zeropage(bus),
        AddrMode::ZeropageX => ga_zeropage_x(bus),
        AddrMode::ZeropageY => ga_zeropage_y(bus),
        AddrMode::Relative  => ga_relative(bus),
        AddrMode::Absolute  => ga_absolute(bus),
        AddrMode::AbsoluteX => ga_absolute_x(bus),
        AddrMode::AbsoluteY => ga_absolute_y(bus),
        AddrMode::Indirect  => ga_indirect(bus),
        AddrMode::IndirectX => ga_indirect_x(bus),
        AddrMode::IndirectY => ga_indirect_y(bus),
        _ => 0
    };
}

fn handle_super_addressing<T : Bus>(bus : &mut T){
    let super_instruction = get_super_instruction(bus);
    if(super_instruction){
        update_cycles(bus, 1);
    }
}

fn ga_immediate<T : Bus>(bus : &mut T) -> u16 {
    offset_pc(bus, 1)
}
fn ga_zeropage<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let lsb = bus.read_byte(lsb_address);
    utils::join_bytes(0x00, lsb)
}
fn ga_zeropage_x<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let x = get_idx(bus);
    let lsb = bus.read_byte(lsb_address);
    utils::join_bytes(0x00, lsb + x)

}
fn ga_zeropage_y<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let y = get_idy(bus);
    let lsb = bus.read_byte(lsb_address);
    utils::join_bytes(0x00, lsb + y)
}
fn ga_relative<T : Bus>(bus : &mut T) -> u16 {
    let pc= offset_pc(bus, 1);
    let offset = bus.read_byte(pc) as u16;
    let relative_offset = if utils::B7(offset) {0xFF00 | offset} else {offset};
    let (address, page_cross) = utils::page_cross_sum(pc, relative_offset);
    if(page_cross){
        handle_super_addressing(bus);
    }
    address + 1
}
fn ga_absolute<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let msb_address = offset_pc(bus, 1);
    let lsb = bus.read_byte(lsb_address);
    let msb = bus.read_byte(msb_address);
    join_bytes(msb, lsb)
}
fn ga_absolute_x<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let msb_address = offset_pc(bus, 1);
    let lsb = bus.read_byte(lsb_address);
    let msb = bus.read_byte(msb_address);
    let x = get_idx(bus);
    let (address, page_cross) = utils::page_cross_sum(join_bytes(msb, lsb), join_bytes(0x00, x));
    if(page_cross){
        handle_super_addressing(bus);
    }
    address
}
fn ga_absolute_y<T : Bus>(bus : &mut T) -> u16 {
    let lsb_address = offset_pc(bus, 1);
    let msb_address = offset_pc(bus, 1);
    let lsb = bus.read_byte(lsb_address);
    let msb = bus.read_byte(msb_address);
    let y = get_idy(bus);
    let (address, page_cross) = utils::page_cross_sum(join_bytes(msb, lsb), join_bytes(0x00, y));
    if(page_cross){
        handle_super_addressing(bus);
    }
    address
}
fn ga_indirect<T : Bus>(bus : &mut T) -> u16 {
    let lsb1_address = offset_pc(bus, 1);
    let msb1_address = offset_pc(bus, 1);
    let lsb1 = bus.read_byte(lsb1_address);
    let msb1 = bus.read_byte(msb1_address);
    let lsb_address = utils::join_bytes(msb1, lsb1 + 0);
    let msb_address = utils::join_bytes(msb1, lsb1 + 1);
    let lsb = bus.read_byte(lsb_address);
    let msb = bus.read_byte(msb_address);
    utils::join_bytes(msb, lsb)
}
fn ga_indirect_x<T : Bus>(bus : &mut T) -> u16 {
    let table_start_address= offset_pc(bus, 1);
    let table_start = bus.read_byte(table_start_address);
    let table_offset = get_idx(bus);
    let table_address = table_start + table_offset;
    let lsb_address = join_bytes(0x00, table_address + 0);
    let msb_address = join_bytes(0x00, table_address + 1);
    let lsb = bus.read_byte(lsb_address);
    let msb = bus.read_byte(msb_address);
    join_bytes(msb, lsb)
}
fn ga_indirect_y<T : Bus>(bus : &mut T) -> u16 {
    let table_lsb_address= offset_pc(bus, 1);
    let table_lsb = bus.read_byte(table_lsb_address);
    let base_lsb = bus.read_byte(utils::join_bytes(0x00, table_lsb + 0));
    let base_msb = bus.read_byte(utils::join_bytes(0x00, table_lsb + 1));
    let base_address = join_bytes(base_msb, base_lsb);
    let y = get_idy(bus);
    let (address, page_cross) = page_cross_sum(base_address, y as u16);
    if(page_cross){
        handle_super_addressing(bus);
    }
    address
}

pub fn fetch<T : Bus>(bus : &mut T) -> u8 {
    let pc = offset_pc(bus, 1);
    bus.read_byte(pc)
}


pub fn execute<T : Bus>(bus : &mut T, opcode : u8)
{
    match opcode {
        0x03 => {},
        0x00 => {
                    update_cycles(bus, 7);
                    op_brk(bus, AddrMode::Implicit);
                },        
        0x01 => {
                    update_cycles(bus, 6);
                    op_ora(bus, AddrMode::IndirectX);
                },        
        0x05 => {
                    update_cycles(bus, 3);
                    op_ora(bus, AddrMode::Zeropage);
                },        
        0x06 => {
                    update_cycles(bus, 5);
                    op_asl(bus, AddrMode::Zeropage);
                },        
        0x08 => {
                    update_cycles(bus, 3);
                    op_php(bus, AddrMode::Implicit);
                },        
        0x09 => {
                    update_cycles(bus, 2);
                    op_ora(bus, AddrMode::Immediate);
                },        
        0x0A => {
                    update_cycles(bus, 2);
                    op_asl(bus, AddrMode::Accumulator);
                },        
        0x0D => {
                    update_cycles(bus, 4);
                    op_ora(bus, AddrMode::Absolute);
                },        
        0x0E => {
                    update_cycles(bus, 6);
                    op_asl(bus, AddrMode::Absolute);
                },        
        0x10 => {
                    update_cycles(bus, 2);
                    op_bpl(bus, AddrMode::Relative);
                },        
        0x11 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_ora(bus, AddrMode::IndirectY);
                },        
        0x15 => {
                    update_cycles(bus, 4);
                    op_ora(bus, AddrMode::ZeropageX);
                },        
        0x16 => {
                    update_cycles(bus, 6);
                    op_asl(bus, AddrMode::ZeropageX);
                },        
        0x18 => {
                    update_cycles(bus, 2);
                    op_clc(bus, AddrMode::Implicit);
                },        
        0x19 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_ora(bus, AddrMode::AbsoluteY);
                },        
        0x1D => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_ora(bus, AddrMode::AbsoluteX);
                },        
        0x1E => {
                    update_cycles(bus, 7);
                    op_asl(bus, AddrMode::AbsoluteX);
                },        
        0x20 => {
                    update_cycles(bus, 6);
                    op_jsr(bus, AddrMode::Absolute);
                },        
        0x21 => {
                    update_cycles(bus, 6);
                    op_and(bus, AddrMode::IndirectX);
                },        
        0x24 => {
                    update_cycles(bus, 3);
                    op_bit(bus, AddrMode::Zeropage);
                },        
        0x25 => {
                    update_cycles(bus, 3);
                    op_and(bus, AddrMode::Zeropage);
                },        
        0x26 => {
                    update_cycles(bus, 5);
                    op_rol(bus, AddrMode::Zeropage);
                },        
        0x28 => {
                    update_cycles(bus, 4);
                    op_plp(bus, AddrMode::Implicit);
                },        
        0x29 => {
                    update_cycles(bus, 2);
                    op_and(bus, AddrMode::Immediate);
                },        
        0x2A => {
                    update_cycles(bus, 2);
                    op_rol(bus, AddrMode::Accumulator);
                },        
        0x2C => {
                    update_cycles(bus, 4);
                    op_bit(bus, AddrMode::Absolute);
                },        
        0x2D => {
                    update_cycles(bus, 4);
                    op_and(bus, AddrMode::Absolute);
                },        
        0x2E => {
                    update_cycles(bus, 6);
                    op_rol(bus, AddrMode::Absolute);
                },        
        0x30 => {
                    update_cycles(bus, 2);
                    op_bmi(bus, AddrMode::Relative);
                },        
        0x31 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_and(bus, AddrMode::IndirectY);
                },        
        0x35 => {
                    update_cycles(bus, 4);
                    op_and(bus, AddrMode::ZeropageX);
                },        
        0x36 => {
                    update_cycles(bus, 6);
                    op_rol(bus, AddrMode::ZeropageX);
                },        
        0x38 => {
                    update_cycles(bus, 2);
                    op_sec(bus, AddrMode::Implicit);
                },        
        0x39 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_and(bus, AddrMode::AbsoluteY);
                },        
        0x3D => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_and(bus, AddrMode::AbsoluteX);
                },        
        0x3E => {
                    update_cycles(bus, 7);
                    op_rol(bus, AddrMode::AbsoluteX);
                },        
        0x40 => {
                    update_cycles(bus, 6);
                    op_rti(bus, AddrMode::Implicit);
                },        
        0x41 => {
                    update_cycles(bus, 6);
                    op_eor(bus, AddrMode::IndirectX);
                },        
        0x45 => {
                    update_cycles(bus, 3);
                    op_eor(bus, AddrMode::Zeropage);
                },        
        0x46 => {
                    update_cycles(bus, 5);
                    op_lsr(bus, AddrMode::Zeropage);
                },        
        0x48 => {
                    update_cycles(bus, 3);
                    op_pha(bus, AddrMode::Implicit);
                },        
        0x49 => {
                    update_cycles(bus, 2);
                    op_eor(bus, AddrMode::Immediate);
                },        
        0x4A => {
                    update_cycles(bus, 2);
                    op_lsr(bus, AddrMode::Accumulator);
                },        
        0x4C => {
                    update_cycles(bus, 3);
                    op_jmp(bus, AddrMode::Absolute);
                },        
        0x4D => {
                    update_cycles(bus, 4);
                    op_eor(bus, AddrMode::Absolute);
                },        
        0x4E => {
                    update_cycles(bus, 6);
                    op_lsr(bus, AddrMode::Absolute);
                },        
        0x50 => {
                    update_cycles(bus, 2);
                    op_bvc(bus, AddrMode::Relative);
                },        
        0x51 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_eor(bus, AddrMode::IndirectY);
                },        
        0x55 => {
                    update_cycles(bus, 4);
                    op_eor(bus, AddrMode::ZeropageX);
                },        
        0x56 => {
                    update_cycles(bus, 6);
                    op_lsr(bus, AddrMode::ZeropageX);
                },        
        0x58 => {
                    update_cycles(bus, 2);
                    op_cli(bus, AddrMode::Implicit);
                },        
        0x59 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_eor(bus, AddrMode::AbsoluteY);
                },        
        0x5D => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_eor(bus, AddrMode::AbsoluteX);
                },        
        0x5E => {
                    update_cycles(bus, 7);
                    op_lsr(bus, AddrMode::AbsoluteX);
                },        
        0x60 => {
                    update_cycles(bus, 6);
                    op_rts(bus, AddrMode::Implicit);
                },        
        0x61 => {
                    update_cycles(bus, 6);
                    op_adc(bus, AddrMode::IndirectX);
                },        
        0x65 => {
                    update_cycles(bus, 3);
                    op_adc(bus, AddrMode::Zeropage);
                },        
        0x66 => {
                    update_cycles(bus, 5);
                    op_ror(bus, AddrMode::Zeropage);
                },        
        0x68 => {
                    update_cycles(bus, 4);
                    op_pla(bus, AddrMode::Implicit);
                },        
        0x69 => {
                    update_cycles(bus, 2);
                    op_adc(bus, AddrMode::Immediate);
                },        
        0x6A => {
                    update_cycles(bus, 2);
                    op_ror(bus, AddrMode::Accumulator);
                },        
        0x6C => {
                    update_cycles(bus, 5);
                    op_jmp(bus, AddrMode::Indirect);
                },        
        0x6D => {
                    update_cycles(bus, 4);
                    op_adc(bus, AddrMode::Absolute);
                },        
        0x6E => {
                    update_cycles(bus, 6);
                    op_ror(bus, AddrMode::Absolute);
                },        
        0x70 => {
                    update_cycles(bus, 2);
                    op_bvs(bus, AddrMode::Relative);
                },        
        0x71 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_adc(bus, AddrMode::IndirectY);
                },        
        0x75 => {
                    update_cycles(bus, 4);
                    op_adc(bus, AddrMode::ZeropageX);
                },        
        0x76 => {
                    update_cycles(bus, 6);
                    op_ror(bus, AddrMode::ZeropageX);
                },        
        0x78 => {
                    update_cycles(bus, 2);
                    op_sei(bus, AddrMode::Implicit);
                },        
        0x79 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_adc(bus, AddrMode::AbsoluteY);
                },        
        0x7D => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_adc(bus, AddrMode::AbsoluteX);
                },        
        0x7E => {
                    update_cycles(bus, 7);
                    op_ror(bus, AddrMode::AbsoluteX);
                },        
        0x81 => {
                    update_cycles(bus, 6);
                    op_sta(bus, AddrMode::IndirectX);
                },        
        0x84 => {
                    update_cycles(bus, 3);
                    op_sty(bus, AddrMode::Zeropage);
                },        
        0x85 => {
                    update_cycles(bus, 3);
                    op_sta(bus, AddrMode::Zeropage);
                },        
        0x86 => {
                    update_cycles(bus, 3);
                    op_stx(bus, AddrMode::Zeropage);
                },        
        0x88 => {
                    update_cycles(bus, 2);
                    op_dey(bus, AddrMode::Implicit);
                },        
        0x8A => {
                    update_cycles(bus, 2);
                    op_txa(bus, AddrMode::Implicit);
                },        
        0x8C => {
                    update_cycles(bus, 4);
                    op_sty(bus, AddrMode::Absolute);
                },        
        0x8D => {
                    update_cycles(bus, 4);
                    op_sta(bus, AddrMode::Absolute);
                },        
        0x8E => {
                    update_cycles(bus, 4);
                    op_stx(bus, AddrMode::Absolute);
                },        
        0x90 => {
                    update_cycles(bus, 2);
                    op_bcc(bus, AddrMode::Relative);
                },        
        0x91 => {
                    update_cycles(bus, 6);
                    op_sta(bus, AddrMode::IndirectY);
                },        
        0x94 => {
                    update_cycles(bus, 4);
                    op_sty(bus, AddrMode::ZeropageX);
                },        
        0x95 => {
                    update_cycles(bus, 4);
                    op_sta(bus, AddrMode::ZeropageX);
                },        
        0x96 => {
                    update_cycles(bus, 4);
                    op_stx(bus, AddrMode::ZeropageY);
                },        
        0x98 => {
                    update_cycles(bus, 2);
                    op_tya(bus, AddrMode::Implicit);
                }, 
        0x99 => {
                    update_cycles(bus, 5);
                    op_sta(bus, AddrMode::AbsoluteY);
                },        
        0x9A => {
                    update_cycles(bus, 2);
                    op_txs(bus, AddrMode::Implicit);
                },        
        0x9D => {
                    update_cycles(bus, 5);
                    op_sta(bus, AddrMode::AbsoluteX);
                },        
        0xA0 => {
                    update_cycles(bus, 2);
                    op_ldy(bus, AddrMode::Immediate);
                },        
        0xA1 => {
                    update_cycles(bus, 6);
                    op_lda(bus, AddrMode::IndirectX);
                },        
        0xA2 => {
                    update_cycles(bus, 2);
                    op_ldx(bus, AddrMode::Immediate);
                },        
        0xA4 => {
                    update_cycles(bus, 3);
                    op_ldy(bus, AddrMode::Zeropage);
                },        
        0xA5 => {
                    update_cycles(bus, 3);
                    op_lda(bus, AddrMode::Zeropage);
                },        
        0xA6 => {
                    update_cycles(bus, 3);
                    op_ldx(bus, AddrMode::Zeropage);
                },        
        0xA8 => {
                    update_cycles(bus, 2);
                    op_tay(bus, AddrMode::Implicit);
                },        
        0xA9 => {
                    update_cycles(bus, 2);
                    op_lda(bus, AddrMode::Immediate);
                },        
        0xAA => {
                    update_cycles(bus, 2);
                    op_tax(bus, AddrMode::Implicit);
                },        
        0xAC => {
                    update_cycles(bus, 4);
                    op_ldy(bus, AddrMode::Absolute);
                },        
        0xAD => {
                    update_cycles(bus, 4);
                    op_lda(bus, AddrMode::Absolute);
                },        
        0xAE => {
                    update_cycles(bus, 4);
                    op_ldx(bus, AddrMode::Absolute);
                },        
        0xB0 => {
                    update_cycles(bus, 2);
                    op_bcs(bus, AddrMode::Relative);
                },        
        0xB1 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_lda(bus, AddrMode::IndirectY);
                },        
        0xB4 => {
                    update_cycles(bus, 4);
                    op_ldy(bus, AddrMode::ZeropageX);
                },        
        0xB5 => {
                    update_cycles(bus, 4);
                    op_lda(bus, AddrMode::ZeropageX);
                },        
        0xB6 => {
                    update_cycles(bus, 4);
                    op_ldx(bus, AddrMode::ZeropageY);
                },        
        0xB8 => {
                    update_cycles(bus, 2);
                    op_clv(bus, AddrMode::Implicit);
                },        
        0xB9 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_lda(bus, AddrMode::AbsoluteY);
                },        
        0xBA => {
                    update_cycles(bus, 2);
                    op_tsx(bus, AddrMode::Implicit);
                },        
        0xBC => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_ldy(bus, AddrMode::AbsoluteX);
                },        
        0xBD => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_lda(bus, AddrMode::AbsoluteX);
                },        
        0xBE => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_ldx(bus, AddrMode::AbsoluteY);
                },        
        0xC0 => {
                    update_cycles(bus, 2);
                    op_cpy(bus, AddrMode::Immediate);
                },        
        0xC1 => {
                    update_cycles(bus, 6);
                    op_cmp(bus, AddrMode::IndirectX);
                },        
        0xC4 => {
                    update_cycles(bus, 3);
                    op_cpy(bus, AddrMode::Zeropage);
                },        
        0xC5 => {
                    update_cycles(bus, 3);
                    op_cmp(bus, AddrMode::Zeropage);
                },        
        0xC6 => {
                    update_cycles(bus, 5);
                    op_dec(bus, AddrMode::Zeropage);
                },        
        0xC8 => {
                    update_cycles(bus, 2);
                    op_iny(bus, AddrMode::Implicit);
                },        
        0xC9 => {
                    update_cycles(bus, 2);
                    op_cmp(bus, AddrMode::Immediate);
                },        
        0xCA => {
                    update_cycles(bus, 2);
                    op_dex(bus, AddrMode::Implicit);
                },        
        0xCC => {
                    update_cycles(bus, 4);
                    op_cpy(bus, AddrMode::Absolute);
                },        
        0xCD => {
                    update_cycles(bus, 4);
                    op_cmp(bus, AddrMode::Absolute);
                },        
        0xCE => {
                    update_cycles(bus, 6);
                    op_dec(bus, AddrMode::Absolute);
                },        
        0xD0 => {
                    update_cycles(bus, 2);
                    op_bne(bus, AddrMode::Relative);
                },        
        0xD1 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_cmp(bus, AddrMode::IndirectY);
                },        
        0xD5 => {
                    update_cycles(bus, 4);
                    op_cmp(bus, AddrMode::ZeropageX);
                },        
        0xD6 => {
                    update_cycles(bus, 6);
                    op_dec(bus, AddrMode::ZeropageX);
                },        
        0xD8 => {
                    update_cycles(bus, 2);
                    op_cld(bus, AddrMode::Implicit);
                },        
        0xD9 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_cmp(bus, AddrMode::AbsoluteY);
                },        
        0xDD => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_cmp(bus, AddrMode::AbsoluteX);
                },        
        0xDE => {
                    update_cycles(bus, 7);
                    op_dec(bus, AddrMode::AbsoluteX);
                },        
        0xE0 => {
                    update_cycles(bus, 2);
                    op_cpx(bus, AddrMode::Immediate);
                },        
        0xE1 => {
                    update_cycles(bus, 6);
                    op_sbc(bus, AddrMode::IndirectX);
                },        
        0xE4 => {
                    update_cycles(bus, 3);
                    op_cpx(bus, AddrMode::Zeropage);
                },        
        0xE5 => {
                    update_cycles(bus, 3);
                    op_sbc(bus, AddrMode::Zeropage);
                },        
        0xE6 => {
                    update_cycles(bus, 5);
                    op_inc(bus, AddrMode::Zeropage);
                },        
        0xE8 => {
                    update_cycles(bus, 2);
                    op_inx(bus, AddrMode::Implicit);
                },        
        0xE9 => {
                    update_cycles(bus, 2);
                    op_sbc(bus, AddrMode::Immediate);
                },        
        0xEA => {
                    update_cycles(bus, 2);
                    op_nop(bus, AddrMode::Implicit);
                },        
        0xEC => {
                    update_cycles(bus, 4);
                    op_cpx(bus, AddrMode::Absolute);
                },        
        0xED => {
                    update_cycles(bus, 4);
                    op_sbc(bus, AddrMode::Absolute);
                },        
        0xEE => {
                    update_cycles(bus, 6);
                    op_inc(bus, AddrMode::Absolute);
                },        
        0xF0 => {
                    update_cycles(bus, 2);
                    op_beq(bus, AddrMode::Relative);
                },        
        0xF1 => {
                    update_cycles(bus, 5);
                    set_super_instruction(bus, true);
                    op_sbc(bus, AddrMode::IndirectY);
                },        
        0xF5 => {
                    update_cycles(bus, 4);
                    op_sbc(bus, AddrMode::ZeropageX);
                },        
        0xF6 => {
                    update_cycles(bus, 6);
                    op_inc(bus, AddrMode::ZeropageX);
                },        
        0xF8 => {
                    update_cycles(bus, 2);
                    op_sed(bus, AddrMode::Implicit);
                },        
        0xF9 => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_sbc(bus, AddrMode::AbsoluteY);
                },        
        0xFD => {
                    update_cycles(bus, 4);
                    set_super_instruction(bus, true);
                    op_sbc(bus, AddrMode::AbsoluteX);
                },        
        0xFE => {
                    update_cycles(bus, 7);
                    op_inc(bus, AddrMode::AbsoluteX);
                },
        _ =>    {
                    update_cycles(bus, 1);
                    op_undefined(bus, AddrMode::Implicit);
                }
    }
}

//fn op_adc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_and<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_asl<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bcc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bcs<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_beq<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bit<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bmi<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bne<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bpl<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_brk<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bvc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_bvs<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_clc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_cld<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_cli<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_clv<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_cmp<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_cpx<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_cpy<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_dec<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_dex<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_dey<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_eor<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_inc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_inx<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_iny<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_jmp<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_jsr<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_lda<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_ldx<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_ldy<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_lsr<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_nop<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_ora<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_pha<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_php<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_pla<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_plp<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_rol<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_ror<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_rti<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_rts<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sbc<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sec<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sed<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sei<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sta<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_stx<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_sty<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_tax<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_tay<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_txs<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_txa<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_tya<T : Bus>(bus : &mut T, address_mode : AddrMode){}
//fn op_undefined<T : Bus>(_bus : &mut T, _address_mode : AddrMode) {}

fn op_adc<T : Bus>(bus : &mut T, address_mode : AddrMode){ // TODO
	match address_mode {
		AddrMode::Implicit |
		AddrMode::Accumulator |
		AddrMode::ZeropageY |
		AddrMode::Relative |
		AddrMode::Indirect => {} // Non-supported addressing mode
		_ => {} // END
	}
}
fn op_and<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let oldacc = get_acc(bus);
    let acc = get_acc(bus) & byte;
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
    set_acc(bus, acc);
}
fn op_asl<T : Bus>(bus : &mut T, address_mode : AddrMode){
    match address_mode {
       AddrMode::Accumulator => {
           let byte = get_acc(bus);
           let shifted_byte = byte << 1;
           set_flag(bus, Flag::Carry, utils::b7(byte));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           set_acc(bus, shifted_byte);
       } 
       _ => {
           let address = get_address(bus, address_mode);
           let byte = bus.read_byte(address);
           let shifted_byte = byte << 1;
           set_flag(bus, Flag::Carry, utils::b7(byte));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           bus.write_byte(address, shifted_byte);
       }
    };
}
fn op_bcc<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let carry_flag = get_flag(bus, Flag::Carry);
    if(carry_flag){
        offset_pc(bus, 1);
    }
    else {
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
}
fn op_bcs<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let carry_flag = get_flag(bus, Flag::Carry);
    if(carry_flag){
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
    else {
        offset_pc(bus, 1);
    }

}
fn op_beq<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let zero_flag = get_flag(bus, Flag::Zero);
    if(zero_flag){
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
    else {
        offset_pc(bus, 1);
    }

}
fn op_bit<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let bit = byte & get_acc(bus);
    set_flag(bus, Flag::Zero, bit == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
    set_flag(bus, Flag::Overflow, utils::b6(byte));
}
fn op_bmi<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let negative_flag = get_flag(bus, Flag::Negative);
    if(negative_flag){
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
    else {
        offset_pc(bus, 1);
    }
}
fn op_bne<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let zero_flag = get_flag(bus, Flag::Zero);
    if(zero_flag){
        offset_pc(bus, 1);
    }
    else {
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
}
fn op_bpl<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let negative_flag = get_flag(bus, Flag::Negative);
    if(negative_flag){
        offset_pc(bus, 1);
    }
    else {
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
}
fn op_brk<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let pc = get_pc(bus);
    let (pcmb, pclb) = utils::split_bytes(pc + 1);
    let ps = utils::p4(get_ps(bus), true);
    write_to_stack(bus, pcmb);
    write_to_stack(bus, pclb);
    write_to_stack(bus, ps);
    let irq_lsb = bus.read_byte(0xFFFE); 
    let irq_msb = bus.read_byte(0xFFFF); 
    let jump_address = join_bytes(irq_msb, irq_lsb);
    set_pc(bus, jump_address);
    set_flag(bus, Flag::InterruptDisable, true);

}
fn op_bvc<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let overflow_flag = get_flag(bus, Flag::Overflow);
    if(overflow_flag){
        offset_pc(bus, 1);
    }
    else {
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
}
fn op_bvs<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let overflow_flag = get_flag(bus, Flag::Overflow);
    if(overflow_flag){
        update_cycles(bus, 1);
        set_super_instruction(bus, true);
        let address = get_address(bus, address_mode);
        set_pc(bus, address);
    }
    else {
        offset_pc(bus, 1);
    }
}
fn op_clc<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::Carry, false)
}
fn op_cld<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::DecimalMode, false)
}
fn op_cli<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::InterruptDisable, false)
}
fn op_clv<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::Overflow, false)
}
fn op_cmp<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let acc = get_acc(bus);
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let difference = acc - byte;
    set_flag(bus, Flag::Zero, difference == 0);
    set_flag(bus, Flag::Carry, acc >= byte);
    set_flag(bus, Flag::Negative, utils::b7(difference));
}
fn op_cpx<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idx = get_idx(bus);
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let difference = idx - byte;
    set_flag(bus, Flag::Zero, difference == 0);
    set_flag(bus, Flag::Carry, idx >= byte);
    set_flag(bus, Flag::Negative, utils::b7(difference));
}
fn op_cpy<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idy = get_idy(bus);
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let difference = idy - byte;
    set_flag(bus, Flag::Zero, difference == 0);
    set_flag(bus, Flag::Carry, idy >= byte);
    set_flag(bus, Flag::Negative, utils::b7(difference));
}
fn op_dec<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address) - 1;
    bus.write_byte(address, byte);
    set_flag(bus, Flag::Zero, byte == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
}
fn op_dex<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idx = get_idx(bus) - 1;
    set_idx(bus, idx);
    set_flag(bus, Flag::Zero, idx == 0);
    set_flag(bus, Flag::Negative, utils::b7(idx));
}
fn op_dey<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idy = get_idy(bus) - 1;
    set_idy(bus, idy);
    set_flag(bus, Flag::Zero, idy == 0);
    set_flag(bus, Flag::Negative, utils::b7(idy));
}
fn op_eor<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let oldacc = get_acc(bus);
    let acc = get_acc(bus) ^ byte;
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
    set_acc(bus, acc);
}
fn op_inc<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address) + 1;
    bus.write_byte(address, byte);
    set_flag(bus, Flag::Zero, byte == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
}
fn op_inx<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idx = get_idx(bus) + 1;
    set_idx(bus, idx);
    set_flag(bus, Flag::Zero, idx == 0);
    set_flag(bus, Flag::Negative, utils::b7(idx));
}
fn op_iny<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idy = get_idy(bus) + 1;
    set_idy(bus, idy);
    set_flag(bus, Flag::Zero, idy == 0);
    set_flag(bus, Flag::Negative, utils::b7(idy));
}
fn op_jmp<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    set_pc(bus, address);
}
fn op_jsr<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let pc = get_pc(bus);
    let (msb, lsb) = split_bytes(pc + 1);
    write_to_stack(bus, msb);
    let address = get_address(bus, address_mode); // You might feel tempted to perform both stack writes continuously. Don't.
    write_to_stack(bus, lsb);
    set_pc(bus, address);
}
fn op_lda<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    set_acc(bus, byte);
    set_flag(bus, Flag::Zero, byte == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
}
fn op_ldx<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    set_idx(bus, byte);
    set_flag(bus, Flag::Zero, byte == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
}
fn op_ldy<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    set_idy(bus, byte);
    set_flag(bus, Flag::Zero, byte == 0);
    set_flag(bus, Flag::Negative, utils::b7(byte));
}
fn op_lsr<T : Bus>(bus : &mut T, address_mode : AddrMode){
	match address_mode {
       AddrMode::Accumulator => {
           let byte = get_acc(bus);
           let shifted_byte = byte >> 1;
           set_flag(bus, Flag::Carry, utils::b0(byte));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           set_acc(bus, shifted_byte);
       } 
       _ => {
           let address = get_address(bus, address_mode);
           let byte = bus.read_byte(address);
           let shifted_byte = byte >> 1;
           set_flag(bus, Flag::Carry, utils::b0(byte));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           bus.write_byte(address, shifted_byte);
       }
    };
}
fn op_nop<T : Bus>(_bus : &mut T, _address_mode : AddrMode){}
fn op_ora<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = bus.read_byte(address);
    let oldacc = get_acc(bus);
    let acc = get_acc(bus) | byte;
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
    set_acc(bus, acc);
}
fn op_pha<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let acc = get_acc(bus);
    write_to_stack(bus, acc);
}
fn op_php<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let ps = get_ps(bus);
    write_to_stack(bus, utils::p4(ps, true));
}
fn op_pla<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let acc = read_from_stack(bus);
    set_acc(bus, acc);
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
}
fn op_plp<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let mut ps = read_from_stack(bus);
    utils::s5(&mut ps, true);
    utils::s4(&mut ps, false);
    set_ps(bus, ps);
}
fn op_rol<T : Bus>(bus : &mut T, address_mode : AddrMode){
    match address_mode {
       AddrMode::Accumulator => {
           let acc = get_acc(bus);
           let mut shifted_acc = acc << 1;
           utils::s0(&mut shifted_acc, get_flag(bus, Flag::Carry));
           set_flag(bus, Flag::Zero, shifted_acc == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_acc));
           set_flag(bus, Flag::Carry, utils::b7(acc));
           set_acc(bus, shifted_acc);
       } 
       _ => {
           let address = get_address(bus, address_mode);
           let byte = bus.read_byte(address);
           let mut shifted_byte = byte << 1;
           utils::s0(&mut shifted_byte, get_flag(bus, Flag::Carry));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           set_flag(bus, Flag::Carry, utils::b7(byte));
           bus.write_byte(address, shifted_byte);
       }
    }
}
fn op_ror<T : Bus>(bus : &mut T, address_mode : AddrMode){
    match address_mode {
       AddrMode::Accumulator => {
           let acc = get_acc(bus);
           let mut shifted_acc = acc >> 1;
           utils::s7(&mut shifted_acc, get_flag(bus, Flag::Carry));
           set_flag(bus, Flag::Zero, shifted_acc == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_acc));
           set_flag(bus, Flag::Carry, utils::b0(acc));
           set_acc(bus, shifted_acc);
       } 
       _ => {
           let address = get_address(bus, address_mode);
           let byte = bus.read_byte(address);
           let mut shifted_byte = byte >> 1;
           utils::s7(&mut shifted_byte, get_flag(bus, Flag::Carry));
           set_flag(bus, Flag::Zero, shifted_byte == 0);
           set_flag(bus, Flag::Negative, utils::b7(shifted_byte));
           set_flag(bus, Flag::Carry, utils::b0(byte));
           bus.write_byte(address, shifted_byte);
       }
    }
}
fn op_rti<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let mut ps = read_from_stack(bus);
    utils::s5(&mut ps, true);
    utils::s4(&mut ps, false);

    let pclsb = read_from_stack(bus);
    let pcmsb = read_from_stack(bus);
    let pc = utils::join_bytes(pcmsb, pclsb);

    set_ps(bus, ps);
    set_pc(bus, pc);
}
fn op_rts<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let pclsb = read_from_stack(bus);
    let pcmsb = read_from_stack(bus);
    let pc = 1 + utils::join_bytes(pcmsb, pclsb);

    set_pc(bus, pc);
}
fn op_sbc<T : Bus>(bus : &mut T, address_mode : AddrMode){ // TODO
	match address_mode {
		AddrMode::Implicit |
		AddrMode::Accumulator |
		AddrMode::ZeropageY |
		AddrMode::Relative |
		AddrMode::Indirect => {} // Non-supported addressing mode
		_ => {} // END
	}
}
fn op_sec<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::Carry, true);
}
fn op_sed<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::DecimalMode, true);
}
fn op_sei<T : Bus>(bus : &mut T, address_mode : AddrMode){
    set_flag(bus, Flag::InterruptDisable, true);
}
fn op_sta<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = get_acc(bus);
    bus.write_byte(address, byte)
}
fn op_stx<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = get_idx(bus);
    bus.write_byte(address, byte)
}
fn op_sty<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let address = get_address(bus, address_mode);
    let byte = get_idy(bus);
    bus.write_byte(address, byte)
}
fn op_tax<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let acc = get_acc(bus);
    set_idx(bus, acc);
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
}
fn op_tay<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let acc = get_acc(bus);
    set_idy(bus, acc);
    set_flag(bus, Flag::Zero, acc == 0);
    set_flag(bus, Flag::Negative, utils::b7(acc));
}
fn op_tsx<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let sp = get_sp(bus);
    set_idx(bus, sp);
    set_flag(bus, Flag::Zero, sp == 0);
    set_flag(bus, Flag::Negative, utils::b7(sp));
}
fn op_txa<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idx = get_idx(bus);
    set_acc(bus, idx);
    set_flag(bus, Flag::Zero, idx == 0);
    set_flag(bus, Flag::Negative, utils::b7(idx));
}

fn op_txs<T : Bus>(bus : &mut T, address_mode : AddrMode){
    let idx = get_idx(bus);
    set_sp(bus, idx);
}
fn op_tya<T : Bus>(bus : &mut T, address_mode : AddrMode){
   let idy = get_idy(bus);
   set_acc(bus, idy);
   set_flag(bus, Flag::Zero, idy == 0);
   set_flag(bus, Flag::Negative, utils::b7(idy));
}

fn op_undefined<T : Bus>(bus : &mut T, address_mode : AddrMode){}
