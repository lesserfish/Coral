pub trait Bus {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn peek_byte(&self, address: u16) -> u8;
    fn fetch_mos(&mut self) -> &mut Mos;
    fn get_mos(&mut self) -> Mos {
        *self.fetch_mos()
    }
}

#[derive(Copy, Clone, Debug)] 
pub struct Registers {
    pub pc:  u16,
    pub sp:  u8,
    pub acc: u8,
    pub idx: u8,
    pub idy: u8,
    pub ps:  u8,
}

pub enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCmd,
    Overflow,
    Negative
}

#[derive(Copy, Clone, Debug)] 
pub struct Context {
    pub compĺete: bool,
    pub decimal_enabled: bool,
    pub super_instruction: bool,
}

#[derive(Copy, Clone, Debug)] 
pub struct Mos 
{
    pub registers : Registers,
    pub context : Context,
    pub cycles : u64,
    pub clock : u64
}

pub enum AddrMode 
{
    Implicit,
    Accumulator,
    Immediate,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Relative,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY
}

// Creation

pub fn new_mos() -> Mos
{
    return Mos {
            registers : Registers { pc: 0, sp: 0, acc: 0, idx: 0, idy: 0, ps: 0 },
            context : Context { compĺete: true, decimal_enabled: false, super_instruction: false },
            cycles : 0,
            clock : 0
            };
}

