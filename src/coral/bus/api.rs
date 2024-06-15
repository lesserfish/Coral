use std::path::Path;
use crate::bus::types::*;
use crate::mos;
use crate::ppu;
use crate::controller;
use crate::cartridge;
use std::io::Result;

impl Bus {
    pub fn set_controller_a(&mut self, state : u8){
        self.controller_a.write_live(state);
    }
    pub fn set_controller_b(&mut self, state : u8){
        self.controller_b.write_live(state);
    }
    pub fn tick(&mut self){}
    pub fn frame(&mut self){}
    pub fn complete(&mut self){}
    pub fn get_pixel(&self, x : usize, y : usize) -> u8 {
        let address = y * 256 + x;
        self.data.display[address]
    }
}

pub fn load<T : AsRef<Path>>(filepath : T) -> Result<Bus> {
    let context = Context{dma_page: 0, dma_byte: 0, dma_cycle: 0, dma_hold: false, clock: 0};
    let cpu = mos::new();
    let ppu = ppu::new();
    let cart = cartridge::load_cartridge(filepath)?;
    let data = Data { cpu_ram: [0; 0x800], nt_ram: [0; 0x800], pal_ram: [0; 0x20], display: [0x0; 256 * 240] };
    let controller_a = controller::new();
    let controller_b = controller::new();

    let bus = Bus { context, cpu, ppu, cart, data, controller_a, controller_b };
    
    Ok(bus)
}
