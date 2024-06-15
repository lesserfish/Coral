use crate::ppu::types::*;
use crate::ppu::primitive::*;
use crate::utils;

// Helper functions

fn merge_pixel_bits(lsb : u8, msb : u8) -> [u8; 8]{
    let mut output = [0;8];
    let mut l = lsb;
    let mut m = msb;
    for x in 0..8 {
        let msd = if utils::b7(m) {0x2} else {0x0};
        let lsd = if utils::b7(l) {0x1} else {0x0};
        l = l << 1;
        m = m << 1;
        output[x] = msd + lsd;
    }
    output
}


// Background rendering


fn reset_bg_buffer<T : Bus>(bus : &mut T){
    bus.fetch_ppu().bg_buffer = [PixelInfo{color_index: 0, palette_index: 0, priority: Priority::Middle}; 33 * 8];
}

fn get_tile_id<T : Bus>(bus : &mut T) -> u8 {
    let vram = get_vram(bus);
    let address = 0x2000 | (vram & 0x0FFF);
    bus.read_byte(address)
}

// A single byte in the attribute segment at the nametable memory has enough data for 4 tiles.
// This function receives the entire tile, and outputs the corresponding information for the
// required tile.
fn get_local_attribute(tile_x : u16, tile_y : u16, global_attribute : u8) -> u8 {
    let shift_x: usize = if utils::B1(tile_x) {0x02} else {0x00};
    let shift_y: usize = if utils::B1(tile_y) {0x04} else {0x00};
    let shift = shift_x + shift_y;
    (global_attribute >> shift) & 0x03
}

fn nametable_base_address(nametable_x : bool, nametable_y : bool) -> u16{
    let offset_x = if nametable_x {0x400} else {0x00};
    let offset_y = if nametable_y {0x800} else {0x00};
    offset_x + offset_y
}

fn get_tile_attribute<T : Bus>(bus : &mut T) -> u8 {
    let nametable_x = get_v_nametable_x(bus);
    let nametable_y = get_v_nametable_y(bus);
    let tile_x = get_v_coarse_x(bus) as u16;
    let tile_y = get_v_coarse_y(bus) as u16;
    let base_address = nametable_base_address(nametable_x, nametable_y);
    let offset = 8 * (tile_y >> 2) + (tile_x >> 2);

    let address = base_address + offset;
    let global_attribute= bus.read_byte(address);

    get_local_attribute(tile_x, tile_y, global_attribute)
}
fn get_tile_data<T : Bus>(bus: &mut T, tile_id : u16) -> [u8; 8] {
    let bg_pattern = get_control_flag(bus, ControlFlag::PatternBackground);
    let base_address : u16 = if bg_pattern {0x1000} else {0x00};
    let fine_y = get_v_fine_y(bus) as u16;
    let address = base_address + tile_id * 16 + fine_y;
    let lsb = bus.read_byte(address + 0x00);
    let msb = bus.read_byte(address + 0x08);
    merge_pixel_bits(lsb, msb)
}
fn write_to_bg_buffer<T : Bus>(bus: &mut T, tile : usize, colors : [u8; 8], palette_index : u8){
    let mut bg_buffer = bus.fetch_ppu().bg_buffer;
    for x in 0..7 {
       let address = tile * 8 + x; 
       let color_index = colors[x];
       let priority = Priority::Middle;
       let pixel_info = PixelInfo{color_index, palette_index, priority};
       bg_buffer[address] = pixel_info;
    }
}
fn read_from_bg_buffer<T : Bus>(bus: &mut T, screen_x : usize) -> PixelInfo {
    let fine_x = get_fine_x(bus) as usize;
    let address = screen_x + fine_x;
    bus.fetch_ppu().bg_buffer[address]
}
fn pre_render_tile<T : Bus>(bus : &mut T, tile : usize) {
    let tile_id = get_tile_id(bus);
    let palette_index = get_tile_attribute(bus);
    let colors  = get_tile_data(bus, tile_id as u16);

    write_to_bg_buffer(bus, tile, colors, palette_index);
}
fn pre_render_background<T : Bus>(bus : &mut T){
    reset_bg_buffer(bus);
    let render_background = get_mask_flag(bus, MaskFlag::RenderBackground);
    if render_background {
        for tile in 0..32 {
            pre_render_tile(bus, tile);
            increase_coarse_x(bus);
        }
        pre_render_tile(bus, 32);
    }
}

//

fn get_pixel_color<T : Bus>(bus : &mut T, pixel_info : PixelInfo) -> u8{
   let palette_index = pixel_info.palette_index as u16;
   let color_index = pixel_info.color_index as u16;

   let base_address = 0x3F00;
   let offset = 4 * palette_index + color_index;
   
   let address = base_address + offset;
   bus.read_byte(address)
}

fn render<T : Bus>(bus : &mut T, screen_x : usize){
    let screen_y = get_scanline(bus) as usize;
    let bg_pixel_info = read_from_bg_buffer(bus, screen_x);

    let color = get_pixel_color(bus, bg_pixel_info);
    bus.set_pixel((screen_x, screen_y), color)
}

fn pre_render<T : Bus>(bus : &mut T){
    for x in 0..256 {
        render(bus, x);
    }
}

// API

fn handle_pre_render<T : Bus>(bus : &mut T){
    let cycle = get_cycle(bus);
    if cycle == 1 {
        set_status_flag(bus, StatusFlag::VerticalBlank, false);
        set_status_flag(bus, StatusFlag::SpriteZeroHit, false);
        set_sprite_0_x(bus, -1);
        set_sprite_0_hit_position(bus, -1);
        set_sprite_0_alpha(bus, 0);
    }
    else if cycle == 304 {
        transfer_y(bus);
    }
}
fn handle_visible_scanline<T : Bus>(bus : &mut T){
    let cycle = get_cycle(bus);
    if cycle == 1 {
        pre_render_background(bus);
        pre_render(bus);
    }

    if cycle == 257 {
        increase_fine_y(bus);
        transfer_x(bus);
    }
}

fn handle_end_of_frame<T : Bus>(bus : &mut T){
    let scanline = get_scanline(bus);
    let cycle = get_cycle(bus);
    if scanline == 241 && cycle == 1 {
        set_status_flag(bus, StatusFlag::VerticalBlank, true);
        let enable_nmi = get_control_flag(bus, ControlFlag::EnableNMI);
        if enable_nmi {
            bus.trigger_nmi();
        }
    }
}

pub fn tick<T : Bus>(bus : &mut T){
    let scanline = get_scanline(bus);
    let cycle = get_cycle(bus);

    if scanline == -1 {
        handle_pre_render(bus);
    }
    if scanline == 0 && cycle == 0  {
        increase_cycle(bus);
    }
    if scanline >= 0 && scanline < 240 {
        handle_visible_scanline(bus);
    }
    if scanline >= 241 && scanline < 260 {
        handle_end_of_frame(bus);
    }

    increase_cycle(bus);

}
