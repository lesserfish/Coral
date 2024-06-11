use crate::mos::types::*;

pub fn fetch<T : Bus>(_bus : &mut T) -> u8
{
    return 0;
}

pub fn execute<T : Bus>(bus : &mut T, opcode : u8)
{
    match opcode {
        _ => {op_undefined(bus);}
    }
}

fn op_undefined<T : Bus>(_bus : &mut T)
{
}
