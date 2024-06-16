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

// Sprite rendering


fn reset_fg_buffer<T : Bus>(bus : &mut T){
    bus.fetch_ppu().fg_buffer = [PixelInfo{color_index: 0, palette_index: 0, priority: Priority::Unset}; 32 * 8];
}


fn sprite_is_visible<T : Bus>(bus : &mut T, sprite : Sprite) -> bool {
    let screen_y = get_scanline(bus);
    let sprite_height = if get_control_flag(bus, ControlFlag::SpriteSize) { 16 } else { 8 };
    let sprite_y = sprite.y_pos as i32;
    let difference = screen_y - sprite_y;
    difference >= 0 && difference < sprite_height
}

fn get_visible_sprites<T : Bus>(bus : &mut T) -> Vec<Sprite> {
    let mut visible_sprites = vec![];

    for id in 0..64 {
        let sprite = get_sprite(bus, id);

        if sprite_is_visible(bus, sprite) {
            if visible_sprites.len() < 8 {
                visible_sprites.push(sprite);
            }
            else {
                set_status_flag(bus, StatusFlag::SpriteOverflow, true);
                break;
            }
        }
    }

    visible_sprites
}


fn get_sprite_tile_address<T : Bus>(bus: &mut T, sprite : Sprite) -> u16 {
    let long_sprites = get_control_flag(bus, ControlFlag::SpriteSize);
    if long_sprites {
        let sprite_tile = sprite.tile as u16;
        (sprite_tile & 0x01) * 0x1000 + (sprite_tile >> 1) * 0x20
    } else {
        let base_address = if get_control_flag(bus, ControlFlag::PatternSprite) {0x1000} else { 0x0000 };
        let sprite_tile = sprite.tile as u16;
        base_address + sprite_tile * 16
    }
}

fn get_sprite_colors<T : Bus>(bus : &mut T, sprite : Sprite) -> [u8; 8] {
    let screen_y = get_scanline(bus);
    let sprite_y = sprite.y_pos as i32;
    let vertical_flip = get_sprite_flag(sprite, SpriteFlag::SpriteVerticalFlip);
    let base_address = get_sprite_tile_address(bus, sprite);
    let offset = if vertical_flip {7 - (screen_y - sprite_y)} else { screen_y - sprite_y } as u16;
    let lsb = bus.read_byte(base_address + offset + 0x00);
    let msb = bus.read_byte(base_address + offset + 0x08);
    merge_pixel_bits(lsb, msb)
}

fn get_sprite_attribute(sprite : Sprite) -> u8 {
    4 + utils::t2(sprite.attribute)
}

fn get_sprite_priority(sprite : Sprite) -> Priority {
    if get_sprite_flag(sprite, SpriteFlag::SpritePriority) {Priority::Back} else {Priority::Front}
}

fn write_to_fg_buffer<T : Bus>(bus: &mut T, screen_x : usize, colors : [u8; 8], palette_index : u8, priority : Priority){
    for x in 0..8 {
        let address = screen_x + x; 
        let color_index = colors[x];
        let pixel_info = PixelInfo{color_index, palette_index, priority};

        if address < 256{
            let current_priority = bus.fetch_ppu().fg_buffer[address].priority;
            if current_priority == Priority::Unset {
                bus.fetch_ppu().fg_buffer[address] = pixel_info;
            }
        }
    }
}

fn read_from_fg_buffer<T : Bus>(bus: &mut T, screen_x : usize) -> PixelInfo {
    bus.fetch_ppu().fg_buffer[screen_x]
}

fn pre_render_sprite<T : Bus>(bus: &mut T, sprite : Sprite){
    let mut colors = get_sprite_colors(bus, sprite);

    if get_sprite_flag(sprite, SpriteFlag::SpriteHorizontalFlip) {
        colors.reverse();
    }

    let palette_index = get_sprite_attribute(sprite);
    let priority = get_sprite_priority(sprite);
    let screen_x = sprite.x_pos as usize;

    write_to_fg_buffer(bus, screen_x, colors, palette_index, priority);

    if sprite.id == 0 {
        let alpha = utils::flatten_u8(colors);
        set_sprite_0_x(bus, screen_x as i32);
        set_sprite_0_alpha(bus, alpha);
    }
}

fn pre_render_sprites<T : Bus>(bus: &mut T){
    reset_fg_buffer(bus);
    let render_sprites = get_mask_flag(bus, MaskFlag::RenderSprites);
    if render_sprites {
        let visible_sprites = get_visible_sprites(bus);
        for sprite in visible_sprites {
            pre_render_sprite(bus, sprite);
        }
    }

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
    let base_address = 0x23C0 + nametable_base_address(nametable_x, nametable_y);
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
    for x in 0..8 {
        let address = tile * 8 + x; 
        let color_index = colors[x];
        let priority = Priority::Middle;
        let pixel_info = PixelInfo{color_index, palette_index, priority};
        bus.fetch_ppu().bg_buffer[address] = pixel_info;
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

// Sprite 0 Hit Logic

fn rendering_s0<T : Bus>(bus : &mut T, screen_x : i32) -> bool{
    let sprite0_x = get_sprite_0_x(bus);
    if sprite0_x >= 0 {
        let distance = screen_x - sprite0_x;
        distance >= 0 && distance < 8
    } else {
        false
    }
}

fn s0_opaque<T : Bus>(bus : &mut T, screen_x : usize) -> bool {
    let sprite0_x = get_sprite_0_x(bus) as usize;
    let distance = screen_x - sprite0_x;
    let alpha = get_sprite_0_alpha(bus) << distance;
    utils::b7(alpha)
}

fn bg_opaque<T : Bus>(bus : &mut T, screen_x : usize) -> bool {
    let pixel_info = read_from_bg_buffer(bus, screen_x);
    pixel_info.color_index > 0
}
fn precheck_s0<T : Bus>(bus : &mut T, screen_x : usize){
    if rendering_s0(bus, screen_x as i32) {
        if !get_status_flag(bus, StatusFlag::SpriteZeroHit) {
            if s0_opaque(bus, screen_x){
                if bg_opaque(bus, screen_x){
                    if get_mask_flag(bus, MaskFlag::RenderBackground){
                        if get_mask_flag(bus, MaskFlag::RenderSprites){
                            let render_sprites_left = get_mask_flag(bus, MaskFlag::RenderSpritesLeft);
                            let render_background_left = get_mask_flag(bus, MaskFlag::RenderBackgroundLeft);
                            let last_check = if render_sprites_left || render_background_left {
                                screen_x >= 8
                            } else {
                                true
                            };
                            if last_check {
                                set_sprite_0_hit_position(bus, screen_x as i32);
                            }

                        }
                    }
                }
            }
        }
    }
}

fn check_s0<T : Bus>(bus : &mut T){
    let screen_x = get_cycle(bus) - 1;
    let s0_hit = get_sprite_0_hit_position(bus);
    if screen_x == s0_hit {
        set_status_flag(bus, StatusFlag::SpriteZeroHit, true);
    }
}

// Rendering

fn get_pixel_color<T : Bus>(bus : &mut T, pixel_info : PixelInfo) -> u8{
    let palette_index = pixel_info.palette_index as u16;
    let color_index = pixel_info.color_index as u16;

    let base_address = 0x3F00;
    let offset = 4 * palette_index + color_index;

    let address = base_address + offset;
    bus.read_byte(address)
}

fn choose_pixel_info(bg_info : PixelInfo, fg_info : PixelInfo) -> PixelInfo {
    if fg_info.priority == Priority::Unset {
        bg_info
    } else if bg_info.color_index == 0 && fg_info.color_index == 0 {
        PixelInfo{color_index: 0, palette_index: 0, priority: Priority::Middle}
    } else if bg_info.color_index == 0 {
        fg_info
    } else if fg_info.color_index == 0 {
        bg_info
    } else if fg_info.priority == Priority::Front{
        fg_info
    } else if fg_info.priority == Priority::Back{
        bg_info
    } else {
        bg_info
    }
}

fn render<T : Bus>(bus : &mut T, screen_x : usize){
    let screen_y = get_scanline(bus) as usize;
    let bg_pixel_info = read_from_bg_buffer(bus, screen_x);
    let fg_pixel_info = read_from_fg_buffer(bus, screen_x);

    let pixel_info = choose_pixel_info(bg_pixel_info, fg_pixel_info);
    let color = get_pixel_color(bus, pixel_info);
    bus.set_pixel((screen_x, screen_y), color)
}

fn pre_render<T : Bus>(bus : &mut T){
    for x in 0..256 {
        render(bus, x);
        precheck_s0(bus, x);
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
    if cycle >= 1 && cycle < 257 {
        check_s0(bus)
    }
    if cycle == 257 {
        pre_render_sprites(bus);
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
