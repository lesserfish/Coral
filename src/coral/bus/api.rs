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

    fn tick_dma_r(&mut self, offset : u16){
        let page = self.context.dma_page as u16;
        let address = 0x100 * page + offset;
        let byte = mos::Bus::read_byte(self, address);
        self.context.dma_byte = byte;
        self.context.dma_cycle += 1;
    }
    fn tick_dma_w(&mut self, offset : u16){
        let byte = self.context.dma_byte;
        ppu::dma_port(self, offset, byte);
        self.context.dma_cycle += 1;
        if offset == 255 {
            self.context.dma_hold = false;
        }
    }
    fn tick_dma(&mut self){
        let dma_cycle = self.context.dma_cycle;
        let offset = dma_cycle / 2;
        if offset % 2 == 0 {
            self.tick_dma_r(offset as u16);
        } else {
            self.tick_dma_w(offset as u16);
        }
    }
    fn tick_cpu(&mut self){
        let dma_hold = self.context.dma_hold; 
        if dma_hold {
            self.tick_dma();
        } else {
            mos::tick(self);
        }
    }
    fn tick_ppu(&mut self){
        ppu::tick(self);
    }
    pub fn tick(&mut self){
        let clock = self.context.clock;
        if clock % 3 == 0 {
            self.tick_cpu();
        }
        self.tick_ppu();
    }
    pub fn frame(&mut self){
        while !self.ppu.complete() {
            self.tick();
        }
    }
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
