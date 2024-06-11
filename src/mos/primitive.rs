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

pub fn tick<T : Bus>(bus : &mut T)
{
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

fn update_clock<T : Bus>(bus : &mut T, offset : i64)
{
    let cpu_ref = bus.fetch_mos();
    cpu_ref.clock += offset as u64;
}

fn get_clock<T : Bus>(bus : &mut T) -> u64 
{
    let cpu_ref = bus.get_mos();
    return cpu_ref.clock;
}

fn update_cycles<T : Bus>(bus : &mut T, offset : i64)
{
    let cpu_ref = bus.fetch_mos();
    cpu_ref.cycles += offset as u64;
}

fn get_cycles<T : Bus>(bus : &mut T) -> u64 
{
    let cpu_ref = bus.get_mos();
    return cpu_ref.cycles;
}


