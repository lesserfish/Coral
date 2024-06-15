use crate::controller;
use crate::cartridge;
use crate::mos;
use crate::ppu;

#[derive(Copy, Clone, Debug)] 
pub struct Data {
    pub cpu_ram : [u8; 0x800],
    pub nt_ram : [u8; 0x800],
    pub pal_ram : [u8; 0x20],
    pub display : [u8; 256 * 240]
}

#[derive(Copy, Clone, Debug)] 
pub struct Context {
    pub dma_page : u8,
    pub dma_byte : u8,
    pub dma_cycle : i32,
    pub dma_hold : bool,
    pub clock : u64
}

#[derive(Clone)] 
pub struct Bus {
    pub context : Context,
    pub cpu : mos::Mos,
    pub ppu : ppu::PPU,
    pub cart : cartridge::Cartridge,
    pub data : Data,
    pub controller_a : controller::Controller,
    pub controller_b : controller::Controller
}

