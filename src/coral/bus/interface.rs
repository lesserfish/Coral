use crate::bus::types::*;
use crate::cartridge;
use crate::mos;
use crate::ppu;

impl Bus {
    // CPU Read
    fn cpu_read_ram(&mut self, address : u16) -> u8 {
        let mapped_address = address & 0x07FF;
        self.data.cpu_ram[mapped_address as usize]
    }
    fn cpu_read_ppu(&mut self, address : u16) -> u8 {
        let mapped_address = address & 0x7;
        ppu::interface::cpu_read(self, mapped_address)
    }
    fn cpu_read_apu(&mut self, _address : u16) -> u8 {
        0
    }
    fn cpu_read_control(&mut self, address : u16) -> u8 {
        let mapped_address = address & 0x01;
        if mapped_address == 0 {
            self.controller_a.read()
        } else {
            self.controller_b.read()
        }
    }
    fn cpu_read_cart(&mut self, address : u16) -> u8 {
        self.cart.cpu_read(address)
    }

    // CPU Write
    fn cpu_write_ram(&mut self, address : u16, byte : u8) {
        let mapped_address = address & 0x07FF;
        self.data.cpu_ram[mapped_address as usize] = byte;
    }
    fn cpu_write_ppu(&mut self, address : u16, byte : u8) {
        let mapped_address = address & 0x7;
        ppu::interface::cpu_write(self, mapped_address, byte)
    }
    fn cpu_write_apu(&mut self, _address : u16, _byte : u8) {
    }
    fn cpu_write_control(&mut self, address : u16, _byte : u8) {
        let mapped_address = address & 0x01;
        if mapped_address == 0 {
            self.controller_a.write();
        } else {
            self.controller_b.write();
        }
    }
    fn cpu_write_cart(&mut self, address : u16, byte : u8) {
        self.cart.cpu_write(address, byte);
    }
    fn cpu_trigger_dma(&mut self, _address : u16, byte : u8) {
        self.context.dma_page = byte;
        self.context.dma_byte = 0;
        self.context.dma_cycle = 0;
        self.context.dma_hold = true;
    }

    // PPU Read
    fn ppu_read_pt(&mut self, address : u16) -> u8 {
        self.cart.ppu_read(address)
    }
    fn ppu_read_nt(&mut self, address : u16) -> u8 {
        let mirroring = self.cart.header.h_mirroring;
        let nametable_choice = (address & 0x1FFF) >> 10;
        let base_address = if mirroring == cartridge::Mirroring::Horizontal {
            match nametable_choice {
                0 => 0x000,
                1 => 0x400,
                2 => 0x000,
                _ => 0x400
            }
        } else {
            match nametable_choice {
                0 => 0x000,
                1 => 0x000,
                2 => 0x400,
                _ => 0x400
            }
        };
        let mapped_address = base_address + (address & 0x03FF);
        self.data.nt_ram[mapped_address as usize]
    }
    fn ppu_read_pal(&mut self, address : u16) -> u8 {
        let mapped_address = match address & 0x1F {
            0x10 => 0x00,
            0x14 => 0x04,
            0x18 => 0x08,
            0x1C => 0x0C,
            _ => address & 0x1F
        };
        self.data.pal_ram[mapped_address as usize]
    }

    // PPU Write
    fn ppu_write_pt(&mut self, address : u16, byte : u8) {
        self.cart.ppu_write(address, byte);
    }
    fn ppu_write_nt(&mut self, address : u16, byte : u8) {
        let mirroring = self.cart.header.h_mirroring;
        let nametable_choice = (address & 0x1FFF) >> 10;
        let base_address = if mirroring == cartridge::Mirroring::Horizontal {
            match nametable_choice {
                0 => 0x000,
                1 => 0x400,
                2 => 0x000,
                _ => 0x400
            }
        } else {
            match nametable_choice {
                0 => 0x000,
                1 => 0x000,
                2 => 0x400,
                _ => 0x400
            }
        };
        let mapped_address = base_address + (address & 0x03FF);
        self.data.nt_ram[mapped_address as usize] = byte;
    }
    fn ppu_write_pal(&mut self, address : u16, byte : u8) {
        let mapped_address = match address & 0x1F {
            0x10 => 0x00,
            0x14 => 0x04,
            0x18 => 0x08,
            0x1C => 0x0C,
            _ => address & 0x1F
        };
        self.data.pal_ram[mapped_address as usize] = byte;
    }

}

impl mos::Bus for Bus {
    fn read_byte(&mut self, address: u16) -> u8 {
        if address <= 0x1FFF      { self.cpu_read_ram(address) }         // 0x0000 - 0x1FFF
        else if address <= 0x3FFF { self.cpu_read_ppu(address) }         // 0x2000 - 0x3FFF
        else if address <= 0x4015 { self.cpu_read_apu(address) }         // 0x4000 - 0x4015
        else if address <= 0x4017 { self.cpu_read_control(address) }     // 0x4016 - 0x4017
        else if address >= 0x4020 { self.cpu_read_cart(address) }        // 0x4020 - 0xFFFF
        else { 0 }                                                       
    }
    fn write_byte(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF      { self.cpu_write_ram(address, byte)}       // 0x0000 - 0x1FFF
        else if address <= 0x3FFF { self.cpu_write_ppu(address, byte)}       // 0x2000 - 0x3FFF
        else if address == 4014   { self.cpu_trigger_dma(address, byte)}     // 0x4014
        else if address <= 0x4015 { self.cpu_write_apu(address, byte)}       // 0x4000 - 0x4015
        else if address <= 0x4017 { self.cpu_write_control(address, byte)}   // 0x4016 - 0x4017
        else if address >= 0x4020 { self.cpu_write_cart(address, byte)}      // 0x4020 - 0xFFFF
    }
    fn fetch_mos(&mut self) -> &mut mos::Mos {
        &mut self.cpu
    }
}

impl ppu::Bus for Bus {
    fn read_byte(&mut self, address : u16) -> u8 {
        if address <= 0x1FFF { self.ppu_read_pt(address) }
        else if address <= 0x3EFF { self.ppu_read_nt(address) }
        else if address <= 0x3FFF { self.ppu_read_pal(address) }
        else { 0 }
    }
    fn write_byte(&mut self, address : u16, byte : u8){
        if address <= 0x1FFF { self.ppu_write_pt(address, byte) }
        else if address <= 0x3EFF { self.ppu_write_nt(address, byte) }
        else if address <= 0x3FFF { self.ppu_write_pal(address, byte) }
    }
    fn set_pixel(&mut self, (x, y): (u16, u16), color : u8) {
        let address = (y * 256 + x) as usize;
        self.data.display[address] = color; 
    }
    fn trigger_nmi(&mut self) {
        mos::nmi(self);
    }
    fn fetch_ppu(&mut self) -> &mut ppu::PPU{
        &mut self.ppu
    }
}
