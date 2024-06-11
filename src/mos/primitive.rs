use crate::utils;
use crate::mos::types::*;
use crate::mos::instructions::*;


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

pub fn update_clock<T : Bus>(bus : &mut T, offset : i64){
    let cpu_ref = bus.fetch_mos();
    cpu_ref.clock += offset as u64;
}

pub fn get_clock<T : Bus>(bus : &mut T) -> u64 {
    let cpu_ref = bus.get_mos();
    return cpu_ref.clock;
}

pub fn update_cycles<T : Bus>(bus : &mut T, offset : i64){
    let cpu_ref = bus.fetch_mos();
    cpu_ref.cycles += offset as u64;
}

pub fn get_cycles<T : Bus>(bus : &mut T) -> u64 {
    let cpu_ref = bus.get_mos();
    return cpu_ref.cycles;
}

// Registers
// Set
pub fn set_pc<T : Bus>(bus : &mut T, v : u16){
    bus.fetch_mos().registers.pc = v;
}
pub fn set_sp<T : Bus>(bus : &mut T, v : u8){
    bus.fetch_mos().registers.sp = v;
}
pub fn set_acc<T : Bus>(bus : &mut T, v : u8){
    bus.fetch_mos().registers.acc = v;
}
pub fn set_idx<T : Bus>(bus : &mut T, v : u8){
    bus.fetch_mos().registers.idx = v;
}
pub fn set_idy<T : Bus>(bus : &mut T, v : u8){
    bus.fetch_mos().registers.idy = v;
}
pub fn set_ps<T : Bus>(bus : &mut T, v : u8){
    bus.fetch_mos().registers.ps = v;
}

// Get
pub fn get_pc<T : Bus>(bus : &mut T) -> u16 {
    return bus.get_mos().registers.pc;
}
pub fn get_sp<T : Bus>(bus : &mut T) -> u8 {
    return bus.get_mos().registers.sp;
}
pub fn get_acc<T : Bus>(bus : &mut T) -> u8 {
    return bus.get_mos().registers.acc;
}
pub fn get_idx<T : Bus>(bus : &mut T) -> u8 {
    return bus.get_mos().registers.idx;
}
pub fn get_idy<T : Bus>(bus : &mut T) -> u8 {
    return bus.get_mos().registers.idy;
}
pub fn get_ps<T : Bus>(bus : &mut T) -> u8 {
    return bus.get_mos().registers.ps;
}

// Fetch
pub fn fetch_pc<T : Bus>(bus : &mut T) -> &mut u16 {
    return &mut bus.fetch_mos().registers.pc;
}
pub fn fetch_sp<T : Bus>(bus : &mut T) -> &mut u8 {
    return &mut bus.fetch_mos().registers.sp;
}
pub fn fetch_acc<T : Bus>(bus : &mut T) -> &mut u8 {
    return &mut bus.fetch_mos().registers.acc;
}
pub fn fetch_idx<T : Bus>(bus : &mut T) -> &mut u8 {
    return &mut bus.fetch_mos().registers.idx;
}
pub fn fetch_idy<T : Bus>(bus : &mut T) -> &mut u8 {
    return &mut bus.fetch_mos().registers.idy;
}
pub fn fetch_ps<T : Bus>(bus : &mut T) -> &mut u8 {
    return &mut bus.fetch_mos().registers.ps;
}

// Map
pub fn map_pc<T : Bus>(bus : &mut T, f : fn(u16) -> u16) {
    bus.fetch_mos().registers.pc = f(bus.get_mos().registers.pc);
}
pub fn map_sp<T : Bus>(bus : &mut T, f : fn(u8) -> u8) {
    bus.fetch_mos().registers.sp = f(bus.get_mos().registers.sp);
}
pub fn map_acc<T : Bus>(bus : &mut T, f : fn(u8) -> u8) {
    bus.fetch_mos().registers.acc = f(bus.get_mos().registers.acc);
}
pub fn map_idx<T : Bus>(bus : &mut T, f : fn(u8) -> u8) {
    bus.fetch_mos().registers.idx = f(bus.get_mos().registers.idx);
}
pub fn map_idy<T : Bus>(bus : &mut T, f : fn(u8) -> u8) {
    bus.fetch_mos().registers.idy = f(bus.get_mos().registers.idy);
}
pub fn map_ps<T : Bus>(bus : &mut T, f : fn(u8) -> u8) {
    bus.fetch_mos().registers.ps = f(bus.get_mos().registers.ps);
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

