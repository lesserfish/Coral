use crate::coral::utils;
use crate::coral::mos::types::*;
use crate::coral::mos::instructions::*;


//  Some quick words in naming conventions:
//
//  The following functions are mostly getters / setters that instructions will use as building
//  blocks to perform their operations.
//  
//  A setter function, that inherently updates the state of the bus, should be named: set_xxxx
//  A getter function that updates the state of the bus should be named: fetch_xxxx
//  A getter function that does NOT update the state of the bus should be named: get_xxxx
//

pub fn tick<T : Bus>(bus : &mut T){
    update_clock(bus, 1);
    let remaining_cycles = get_cycles(bus);
    if remaining_cycles > 0 {
        update_clock(bus, -1);
    } else {
        let opcode = fetch(bus);
        execute(bus, opcode);
        update_cycles(bus, -1);
    }
}

pub fn update_clock<T : Bus>(bus : &mut T, offset : i64) -> u64{
    let output = bus.get_mos().clock;
    bus.fetch_mos().clock += offset as u64;
    output
}

pub fn get_clock<T : Bus>(bus : &mut T) -> u64 {
    bus.get_mos().clock
}

pub fn update_cycles<T : Bus>(bus : &mut T, offset : i64) -> u64{
    let output = bus.get_mos().cycles;
    bus.fetch_mos().cycles += offset as u64;
    output
}

pub fn get_cycles<T : Bus>(bus : &mut T) -> u64 {
    bus.get_mos().cycles
}

// Registers
// Set
pub fn set_pc<T : Bus>(bus : &mut T, v : u16) -> u16{
    let output = bus.get_mos().registers.pc;
    bus.fetch_mos().registers.pc = v;
    output
}
pub fn set_sp<T : Bus>(bus : &mut T, v : u8) -> u8{
    let output = bus.get_mos().registers.sp;
    bus.fetch_mos().registers.sp = v;
    output
}
pub fn set_acc<T : Bus>(bus : &mut T, v : u8) -> u8{
    let output = bus.get_mos().registers.acc;
    bus.fetch_mos().registers.acc = v;
    output
}
pub fn set_idx<T : Bus>(bus : &mut T, v : u8) -> u8{
    let output = bus.get_mos().registers.idx;
    bus.fetch_mos().registers.idx = v;
    output
}
pub fn set_idy<T : Bus>(bus : &mut T, v : u8) -> u8{
    let output = bus.get_mos().registers.idy;
    bus.fetch_mos().registers.idy = v;
    output
}
pub fn set_ps<T : Bus>(bus : &mut T, v : u8) -> u8{
    let output = bus.get_mos().registers.ps;
    bus.fetch_mos().registers.ps = v;
    output
}

// Get
pub fn get_pc<T : Bus>(bus : &mut T) -> u16 {
    bus.get_mos().registers.pc
}
pub fn get_sp<T : Bus>(bus : &mut T) -> u8 {
    bus.get_mos().registers.sp
}
pub fn get_acc<T : Bus>(bus : &mut T) -> u8 {
    bus.get_mos().registers.acc
}
pub fn get_idx<T : Bus>(bus : &mut T) -> u8 {
    bus.get_mos().registers.idx
}
pub fn get_idy<T : Bus>(bus : &mut T) -> u8 {
    bus.get_mos().registers.idy
}
pub fn get_ps<T : Bus>(bus : &mut T) -> u8 {
    bus.get_mos().registers.ps
}

// Fetch
pub fn fetch_pc<T : Bus>(bus : &mut T) -> &mut u16 {
    &mut bus.fetch_mos().registers.pc
}
pub fn fetch_sp<T : Bus>(bus : &mut T) -> &mut u8 {
    &mut bus.fetch_mos().registers.sp
}
pub fn fetch_acc<T : Bus>(bus : &mut T) -> &mut u8 {
    &mut bus.fetch_mos().registers.acc
}
pub fn fetch_idx<T : Bus>(bus : &mut T) -> &mut u8 {
    &mut bus.fetch_mos().registers.idx
}
pub fn fetch_idy<T : Bus>(bus : &mut T) -> &mut u8 {
    &mut bus.fetch_mos().registers.idy
}
pub fn fetch_ps<T : Bus>(bus : &mut T) -> &mut u8 {
    &mut bus.fetch_mos().registers.ps
}

// Map
pub fn map_pc<T : Bus>(bus : &mut T, f : impl Fn(u16) -> u16) -> u16 {
    let pc = get_pc(bus);
    set_pc(bus, f(pc))
}
pub fn map_sp<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8{
    let sp = get_sp(bus);
    set_sp(bus, f(sp))
}
pub fn map_acc<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8{
    let acc = get_acc(bus);
    set_acc(bus, f(acc))
}
pub fn map_idx<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
    let idx = get_idx(bus);
    set_idx(bus, f(idx))
}
pub fn map_idy<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
    let idy = get_idy(bus);
    set_idy(bus, f(idy))
}
pub fn map_ps<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
    let ps = get_ps(bus);
    set_ps(bus, f(ps))
}

// Set If
pub fn set_pc_if<T : Bus>(bus : &mut T, b : bool, v : u16){
    if b {bus.fetch_mos().registers.pc = v;}
}
pub fn set_sp_if<T : Bus>(bus : &mut T, b : bool, v : u8){
    if b {bus.fetch_mos().registers.sp = v;}
}
pub fn set_acc_if<T : Bus>(bus : &mut T,b : bool, v : u8){
    if b {bus.fetch_mos().registers.acc = v;}
}
pub fn set_idx_if<T : Bus>(bus : &mut T,b : bool, v : u8){
    if b {bus.fetch_mos().registers.idx = v;}
}
pub fn set_idy_if<T : Bus>(bus : &mut T,b : bool, v : u8){
    if b {bus.fetch_mos().registers.idy = v;}
}
pub fn set_ps_if<T : Bus>(bus : &mut T, b : bool, v : u8){
    if b {bus.fetch_mos().registers.ps = v;}
}

pub fn offset_pc<T : Bus>(bus : &mut T, v :  i64) -> u16{
    map_pc(bus, |x| -> u16 {x + v as u16})
}
pub fn offset_sp<T : Bus>(bus : &mut T, v :  i64) -> u8{
    map_sp(bus, |x| -> u8 {x + v as u8})
}
pub fn offset_acc<T : Bus>(bus : &mut T, v : i64) -> u8{
    map_acc(bus, |x| -> u8 {x + v as u8})
}
pub fn offset_idx<T : Bus>(bus : &mut T, v : i64) -> u8{
    map_idx(bus, |x| -> u8 {x + v as u8})
}
pub fn offset_idy<T : Bus>(bus : &mut T, v : i64) -> u8{
    map_idy(bus, |x| -> u8 {x + v as u8})
}
pub fn offset_ps<T : Bus>(bus : &mut T, v :  i64) -> u8{
    map_ps(bus, |x| -> u8 {x + v as u8})
}

// Flags

pub fn get_flag<T : Bus>(bus : &mut T, f : Flag) -> bool {
     return match f {
        Flag::Carry             => utils::b0(get_ps(bus)),
        Flag::Zero              => utils::b1(get_ps(bus)),
        Flag::InterruptDisable  => utils::b2(get_ps(bus)),
        Flag::DecimalMode       => utils::b3(get_ps(bus)),
        Flag::BreakCmd          => utils::b4(get_ps(bus)),
        Flag::Overflow          => utils::b6(get_ps(bus)),
        Flag::Negative          => utils::b7(get_ps(bus))
    };
}

pub fn set_flag<T : Bus>(bus : &mut T, f : Flag, b : bool){
     match f {
        Flag::Carry             => utils::s0(fetch_ps(bus), b),
        Flag::Zero              => utils::s1(fetch_ps(bus), b),
        Flag::InterruptDisable  => utils::s2(fetch_ps(bus), b),
        Flag::DecimalMode       => utils::s3(fetch_ps(bus), b),
        Flag::BreakCmd          => utils::s4(fetch_ps(bus), b),
        Flag::Overflow          => utils::s6(fetch_ps(bus), b),
        Flag::Negative          => utils::s7(fetch_ps(bus), b)
    };
}

// Context

pub fn get_decimal_enabled<T : Bus>(bus : &mut T) -> bool {
    bus.get_mos().context.decimal_enabled
}

pub fn get_complete<T : Bus>(bus : &mut T) -> bool {
    bus.get_mos().context.compĺete
}

pub fn get_super_instruction<T : Bus>(bus : &mut T) -> bool {
    bus.get_mos().context.super_instruction
}

pub fn set_decimal_enabled<T : Bus>(bus : &mut T, v : bool){
    bus.fetch_mos().context.decimal_enabled = v;
}

pub fn set_complete<T : Bus>(bus : &mut T, v : bool){
    bus.fetch_mos().context.compĺete = v;
}

pub fn set_super_instruction<T : Bus>(bus : &mut T, v : bool){
    bus.fetch_mos().context.super_instruction = v; 
}

// Stack

pub fn write_to_stack<T : Bus>(bus : &mut T, byte : u8){
    let sp = offset_sp(bus, -1);
    let address = 0x0100 + utils::join_bytes(0x00, sp);
    bus.write_byte(address, byte);
}

pub fn read_from_stack<T : Bus>(bus : &mut T) -> u8 {
    offset_sp(bus, 1);
    let sp = get_sp(bus);
    let address = 0x0100 + utils::join_bytes(0x00, sp);
    bus.read_byte(address)
}

