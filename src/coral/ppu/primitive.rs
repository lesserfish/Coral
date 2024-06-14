#![allow(unused)]
use crate::ppu::types::*;
use crate::utils;

pub fn get_sprite<T : Bus>(bus : &mut T, id : usize) -> Sprite {
    let y_pos     = bus.fetch_ppu().oam_data[0x04 * id + 0];
    let tile      = bus.fetch_ppu().oam_data[0x04 * id + 1];
    let attribute = bus.fetch_ppu().oam_data[0x04 * id + 2];
    let x_pos     = bus.fetch_ppu().oam_data[0x04 * id + 3];

    Sprite { id, y_pos, tile, attribute, x_pos }
}

// Registers: Setters
pub fn set_control<T : Bus>(bus : &mut T, v : u8) -> u8 {
   let output = bus.get_ppu().registers.control;
   bus.fetch_ppu().registers.control = v;
   output
}
pub fn set_mask<T : Bus>(bus : &mut T, v : u8) -> u8 {
   let output = bus.get_ppu().registers.mask;
   bus.fetch_ppu().registers.mask = v;
   output
}
pub fn set_status<T : Bus>(bus : &mut T, v : u8) -> u8 {
   let output = bus.get_ppu().registers.status;
   bus.fetch_ppu().registers.status = v;
   output
}
pub fn set_fine_x<T : Bus>(bus : &mut T, v : u8) -> u8 {
   let output = bus.get_ppu().registers.fine_x;
   bus.fetch_ppu().registers.fine_x = v;
   output
}
pub fn set_data_buffer<T : Bus>(bus : &mut T, v : u8) -> u8 {
   let output = bus.get_ppu().registers.data_buffer;
   bus.fetch_ppu().registers.data_buffer = v;
   output
}
pub fn set_vram<T : Bus>(bus : &mut T, v : u16) -> u16{
   let output = bus.get_ppu().registers.vram;
   bus.fetch_ppu().registers.vram = v;
   output
}
pub fn set_tram<T : Bus>(bus : &mut T, v : u16) -> u16{
   let output = bus.get_ppu().registers.tram;
   bus.fetch_ppu().registers.tram = v;
   output
}
pub fn set_write_toggle<T : Bus>(bus : &mut T, v : bool) -> bool{
   let output = bus.get_ppu().registers.write_toggle;
   bus.fetch_ppu().registers.write_toggle = v;
   output
}

// Registers: Getters
pub fn get_control<T : Bus>(bus : &mut T) -> u8 {
   bus.get_ppu().registers.control
}
pub fn get_mask<T : Bus>(bus : &mut T) -> u8 {
   bus.get_ppu().registers.mask
}
pub fn get_status<T : Bus>(bus : &mut T) -> u8 {
   bus.get_ppu().registers.status
}
pub fn get_fine_x<T : Bus>(bus : &mut T) -> u8 {
   bus.get_ppu().registers.fine_x
}
pub fn get_data_buffer<T : Bus>(bus : &mut T) -> u8 {
   bus.get_ppu().registers.data_buffer
}
pub fn get_vram<T : Bus>(bus : &mut T) -> u16{
   bus.get_ppu().registers.vram
}
pub fn get_tram<T : Bus>(bus : &mut T) -> u16{
   bus.get_ppu().registers.tram
}
pub fn get_write_toggle<T : Bus>(bus : &mut T) -> bool{
   bus.get_ppu().registers.write_toggle
}

// Registers: Getters
pub fn fetch_control<T : Bus>(bus : &mut T) -> &mut u8 {
   &mut bus.fetch_ppu().registers.control
}
pub fn fetch_mask<T : Bus>(bus : &mut T) -> &mut u8 {
   &mut bus.fetch_ppu().registers.mask
}
pub fn fetch_status<T : Bus>(bus : &mut T) -> &mut u8 {
   &mut bus.fetch_ppu().registers.status
}
pub fn fetch_fine_x<T : Bus>(bus : &mut T) -> &mut u8 {
   &mut bus.fetch_ppu().registers.fine_x
}
pub fn fetch_data_buffer<T : Bus>(bus : &mut T) -> &mut u8 {
   &mut bus.fetch_ppu().registers.data_buffer
}
pub fn fetch_vram<T : Bus>(bus : &mut T) -> &mut u16{
   &mut bus.fetch_ppu().registers.vram
}
pub fn fetch_tram<T : Bus>(bus : &mut T) -> &mut u16{
   &mut bus.fetch_ppu().registers.tram
}
pub fn fetch_write_toggle<T : Bus>(bus : &mut T) -> &mut bool{
   &mut bus.fetch_ppu().registers.write_toggle
}


// Registers: Mappers
pub fn map_control<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
   let x = bus.get_ppu().registers.control;
   set_control(bus, f(x))
}
pub fn map_mask<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
   let x = bus.get_ppu().registers.mask;
   set_mask(bus, f(x))
}
pub fn map_status<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
   let x = bus.get_ppu().registers.status;
   set_status(bus, f(x))
}
pub fn map_fine_x<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
   let x = bus.get_ppu().registers.fine_x;
   set_fine_x(bus, f(x))
}
pub fn map_data_buffer<T : Bus>(bus : &mut T, f : impl Fn(u8) -> u8) -> u8 {
   let x = bus.get_ppu().registers.data_buffer;
   set_data_buffer(bus, f(x))
}
pub fn map_vram<T : Bus>(bus : &mut T, f : impl Fn(u16) -> u16) -> u16 {
   let x = bus.get_ppu().registers.vram;
   set_vram(bus, f(x))
}
pub fn map_tram<T : Bus>(bus : &mut T, f : impl Fn(u16) -> u16) -> u16 {
   let x = bus.get_ppu().registers.tram;
   set_tram(bus, f(x))
}
pub fn map_write_toggle<T : Bus>(bus : &mut T, f : impl Fn(bool) -> bool) -> bool {
   let x = bus.get_ppu().registers.write_toggle;
   set_write_toggle(bus, f(x))
}

// Control Flags
pub fn get_control_flag<T : Bus>(bus : &mut T, f : ControlFlag) -> bool {
    let control = get_control(bus);
    match f {
        ControlFlag::NametableX =>        { utils::b0(control) }
        ControlFlag::NametableY =>        { utils::b1(control) }
        ControlFlag::IncrementMode =>     { utils::b2(control) }
        ControlFlag::PatternSprite =>     { utils::b3(control) }
        ControlFlag::PatternBackground => { utils::b4(control) }
        ControlFlag::SpriteSize =>        { utils::b5(control) }
        ControlFlag::SlaveMode =>         { utils::b6(control) }
        ControlFlag::EnableNMI =>         { utils::b7(control) }
    }
}

pub fn set_control_flag<T : Bus>(bus : &mut T, f : ControlFlag, v: bool){
    let mut control = fetch_control(bus);
    match f {
        ControlFlag::NametableX =>        { utils::s0(control, v) }
        ControlFlag::NametableY =>        { utils::s1(control, v) }
        ControlFlag::IncrementMode =>     { utils::s2(control, v) }
        ControlFlag::PatternSprite =>     { utils::s3(control, v) }
        ControlFlag::PatternBackground => { utils::s4(control, v) }
        ControlFlag::SpriteSize =>        { utils::s5(control, v) }
        ControlFlag::SlaveMode =>         { utils::s6(control, v) }
        ControlFlag::EnableNMI =>         { utils::s7(control, v) }
    }
}

// Mask Flags

pub fn get_mask_flag<T : Bus>(bus : &mut T, f : MaskFlag) -> bool {
    let mask = get_status(bus);
    match f {
        MaskFlag::Grayscale =>            { utils::b0(mask) }
        MaskFlag::RenderBackgroundLeft => { utils::b1(mask) }
        MaskFlag::RenderSpritesLeft =>    { utils::b2(mask) }
        MaskFlag::RenderBackground =>     { utils::b3(mask) }
        MaskFlag::RenderSprites =>        { utils::b4(mask) }
        MaskFlag::EnhancedRed =>          { utils::b5(mask) }
        MaskFlag::EnhanceGreen =>         { utils::b6(mask) }
        MaskFlag::EnhanceBlue =>          { utils::b7(mask) }
    }
}

pub fn set_mask_flag<T : Bus>(bus : &mut T, f : MaskFlag, v : bool){
    let mut mask = fetch_status(bus);
    match f {
        MaskFlag::Grayscale =>            { utils::s0(mask, v) }
        MaskFlag::RenderBackgroundLeft => { utils::s1(mask, v) }
        MaskFlag::RenderSpritesLeft =>    { utils::s2(mask, v) }
        MaskFlag::RenderBackground =>     { utils::s3(mask, v) }
        MaskFlag::RenderSprites =>        { utils::s4(mask, v) }
        MaskFlag::EnhancedRed =>          { utils::s5(mask, v) }
        MaskFlag::EnhanceGreen =>         { utils::s6(mask, v) }
        MaskFlag::EnhanceBlue =>          { utils::s7(mask, v) }
    }
}

// Status Flags

pub fn get_status_flag<T : Bus>(bus : &mut T, f: StatusFlag) -> bool {
    let status = get_status(bus);
    match f {
        StatusFlag::SpriteOverflow =>  { utils::b5(status) }
        StatusFlag::SpriteZeroHit  =>  { utils::b6(status) }
        StatusFlag::VerticalBlank  =>  { utils::b7(status) }
    }
}

pub fn set_status_flag<T : Bus>(bus : &mut T, f: StatusFlag, v : bool){
    let mut status = fetch_status(bus);
    match f {
        StatusFlag::SpriteOverflow =>  { utils::s5(status, v) }
        StatusFlag::SpriteZeroHit  =>  { utils::s6(status, v) }
        StatusFlag::VerticalBlank  =>  { utils::s7(status, v) }
    }
}


// Loopy: VRAM

pub fn get_v_coarse_x<T : Bus>(bus : &mut T) -> u8{
    let vram = get_vram(bus);
    utils::T5(vram) as u8
}
pub fn get_v_coarse_y<T : Bus>(bus : &mut T) -> u8{
    let vram = get_vram(bus);
    utils::T5(vram >> 5) as u8
}
pub fn get_v_nametable_x<T : Bus>(bus : &mut T) -> bool {
    let vram = get_vram(bus);
    utils::B10(vram)
}
pub fn get_v_nametable_y<T : Bus>(bus : &mut T) -> bool {
    let vram = get_vram(bus);
    utils::B11(vram)
}
pub fn get_v_fine_y<T : Bus>(bus : &mut T) -> u8{
    let vram = get_vram(bus);
    utils::T3(vram >> 12) as u8
}
pub fn invert_v_nametable_x<T : Bus>(bus : &mut T){
    let nametable_x = get_v_nametable_x(bus);
    set_v_nametable_x(bus, !nametable_x);
}
pub fn invert_v_nametable_y<T : Bus>(bus : &mut T){
    let nametable_y = get_v_nametable_y(bus);
    set_v_nametable_y(bus, !nametable_y);
}

// 
pub fn set_v_coarse_x<T : Bus>(bus : &mut T, v: u8){
    let vram = get_vram(bus);
    let mask = !utils::T5(0xFFFF); // 0b1111111111100000
    let expose = v as u16;
    set_vram(bus, (vram & mask) | expose);
}
pub fn set_v_coarse_y<T : Bus>(bus : &mut T, v: u8){
    let vram = get_vram(bus);
    let mask = !(utils::T5(0xFFFF) << 5); // 0b1111110000011111
    let expose = (v as u16) << 5;
    set_vram(bus, (vram & mask) | expose);
}
pub fn set_v_nametable_x<T : Bus>(bus : &mut T, v : bool) {
    let vram = get_vram(bus);
    set_vram(bus, utils::P10(vram, v));
}
pub fn set_v_nametable_y<T : Bus>(bus : &mut T, v : bool) {
    let vram = get_vram(bus);
    set_vram(bus, utils::P11(vram, v));
}
pub fn set_v_fine_y<T : Bus>(bus : &mut T, v: u8) {
    let vram = get_vram(bus);
    let mask = !(utils::T3(0xFFFF) << 12); //0b1000111111111111
    let expose = (v as u16) << 12;
    set_vram(bus, (vram & mask) | expose);
}


// Loopy: TRAM

pub fn get_t_coarse_x<T : Bus>(bus : &mut T) -> u8{
    let tram = get_tram(bus);
    utils::T5(tram) as u8
}
pub fn get_t_coarse_y<T : Bus>(bus : &mut T) -> u8{
    let tram = get_tram(bus);
    utils::T5(tram >> 5) as u8
}
pub fn get_t_nametable_x<T : Bus>(bus : &mut T) -> bool {
    let tram = get_tram(bus);
    utils::B10(tram)
}
pub fn get_t_nametable_y<T : Bus>(bus : &mut T) -> bool {
    let tram = get_tram(bus);
    utils::B11(tram)
}
pub fn get_t_fine_y<T : Bus>(bus : &mut T) -> u8{
    let tram = get_tram(bus);
    utils::T3(tram >> 12) as u8
}

pub fn set_t_coarse_x<T : Bus>(bus : &mut T, v: u8){
    let tram = get_tram(bus);
    let mask = !utils::T5(0xFFFF);
    let expose = v as u16;
    set_tram(bus, (tram & mask) | expose);
}
pub fn set_t_coarse_y<T : Bus>(bus : &mut T, v: u8){
    let tram = get_tram(bus);
    let mask = !(utils::T5(0xFFFF) << 5);
    let expose = (v as u16) << 5;
    set_tram(bus, (tram & mask) | expose);
}
pub fn set_t_nametable_x<T : Bus>(bus : &mut T, v : bool) {
    let tram = get_tram(bus);
    set_tram(bus, utils::P10(tram, v));
}
pub fn set_t_nametable_y<T : Bus>(bus : &mut T, v : bool) {
    let tram = get_tram(bus);
    set_tram(bus, utils::P11(tram, v));
}
pub fn set_t_fine_y<T : Bus>(bus : &mut T, v: u8) {
    let tram = get_tram(bus);
    let mask = !(utils::T3(0xFFFF) << 12); 
    let expose = (v as u16) << 12;
    set_tram(bus, (tram & mask) | expose);
}

// Sprite Flag

pub fn get_sprite_flag(sprite : Sprite, f : SpriteFlag) -> bool {
    match f {
        SpriteFlag::SpritePriority =>       { utils::b5(sprite.attribute) }
        SpriteFlag::SpriteHorizontalFlip => { utils::b6(sprite.attribute) }
        SpriteFlag::SpriteVerticalFlip =>   { utils::b7(sprite.attribute) }
    }
}

pub fn get_sprite_palette(sprite : Sprite) -> u8 {
    4 + utils::t2(sprite.attribute)
}

// Context

pub fn set_complete<T : Bus>(bus : &mut T, v : bool) -> bool {
    let output = bus.get_ppu().context.complete;
    bus.fetch_ppu().context.complete = v;
    output
}
pub fn set_scanline<T : Bus>(bus : &mut T, v : i32) -> i32 {
    let output = bus.get_ppu().context.cycle;
    bus.fetch_ppu().context.scanline= v;
    output
}
pub fn set_cycle<T : Bus>(bus : &mut T, v : i32) -> i32 {
    let output = bus.get_ppu().context.cycle;
    bus.fetch_ppu().context.cycle = v;
    output
}
pub fn set_sprite_0_alpha<T : Bus>(bus : &mut T, v : u8) -> u8 {
    let output = bus.get_ppu().context.sprite_0_alpha;
    bus.fetch_ppu().context.sprite_0_alpha = v;
    output
}
pub fn set_sprite_0_x<T : Bus>(bus : &mut T, v : i32) -> i32 {
    let output = bus.get_ppu().context.sprite_0_x;
    bus.fetch_ppu().context.sprite_0_x = v;
    output
}
pub fn set_sprite_0_hit_position<T : Bus>(bus : &mut T, v : i32) -> i32 {
    let output = bus.get_ppu().context.sprite_0_hit_position;
    bus.fetch_ppu().context.sprite_0_hit_position = v;
    output
}


pub fn get_complete<T : Bus>(bus : &mut T) -> bool {
    bus.get_ppu().context.complete
}
pub fn get_scanline<T : Bus>(bus : &mut T) -> i32 {
    bus.get_ppu().context.cycle
}
pub fn get_cycle<T : Bus>(bus : &mut T) -> i32 {
    bus.get_ppu().context.cycle
}
pub fn get_sprite_0_alpha<T : Bus>(bus : &mut T) -> u8 {
    bus.get_ppu().context.sprite_0_alpha
}
pub fn get_sprite_0_x<T : Bus>(bus : &mut T) -> i32 {
    bus.get_ppu().context.sprite_0_x
}
pub fn get_sprite_0_hit_position<T : Bus>(bus : &mut T) -> i32 {
    bus.get_ppu().context.sprite_0_hit_position
}


pub fn increase_coarse_x<T : Bus>(bus : &mut T){
    let render_background = get_mask_flag(bus, MaskFlag::RenderBackground);
    let render_sprites = get_mask_flag(bus, MaskFlag::RenderSprites);

    if render_background || render_sprites {
        let coarse_x = get_v_coarse_x(bus);
        if coarse_x < 31 {
            set_v_coarse_x(bus, coarse_x + 1)
        } else {
            set_v_coarse_x(bus, 0);
            invert_v_nametable_x(bus);
        }
    }
}

pub fn increase_coarse_y<T : Bus>(bus : &mut T){
    let coarse_y = get_v_coarse_y(bus);

    if coarse_y == 29 {
        set_v_coarse_y(bus, 0);
        invert_v_nametable_y(bus);
    } else if coarse_y == 31 {
        set_v_coarse_y(bus, 0);
    } else {
        set_v_coarse_y(bus, coarse_y + 1);
    }
}

pub fn increase_fine_y<T : Bus>(bus : &mut T){
    let render_background = get_mask_flag(bus, MaskFlag::RenderBackground);
    let render_sprites = get_mask_flag(bus, MaskFlag::RenderSprites);

    if render_background || render_sprites {
        let fine_y = get_v_fine_y(bus);
        if fine_y < 7 {
            set_v_fine_y(bus, fine_y + 1)
        } else {
            set_v_fine_y(bus, 0);
            increase_coarse_y(bus);
        }
    }
}

pub fn increase_scanline<T : Bus>(bus : &mut T){
    let scanline = get_scanline(bus);
    if scanline >= 260 {
        set_scanline(bus, -1);
        set_complete(bus, true);
    } else {
        set_scanline(bus, scanline + 1);
    }
}

pub fn increase_cycle<T : Bus>(bus : &mut T){
    let cycle = get_cycle(bus);
    if cycle >= 340 {
        set_cycle(bus, 0);
        increase_scanline(bus);
    } else {
        set_cycle(bus, cycle + 1);
    }
}

pub fn transfer_x<T : Bus>(bus : &mut T){
    let render_background = get_mask_flag(bus, MaskFlag::RenderBackground);
    let render_sprites = get_mask_flag(bus, MaskFlag::RenderSprites);

    if render_background || render_sprites {
        let nametable_x = get_t_nametable_x(bus);
        let coarse_x = get_t_coarse_x(bus);

        set_v_nametable_x(bus, nametable_x);
        set_v_coarse_x(bus, coarse_x);
    }
}


pub fn transfer_y<T : Bus>(bus : &mut T){
    let render_background = get_mask_flag(bus, MaskFlag::RenderBackground);
    let render_sprites = get_mask_flag(bus, MaskFlag::RenderSprites);

    if render_background || render_sprites {
        let nametable_y = get_t_nametable_y(bus);
        let coarse_y = get_t_coarse_y(bus);
        let fine_y = get_t_fine_y(bus);

        set_v_nametable_y(bus, nametable_y);
        set_v_coarse_y(bus, coarse_y);
        set_v_fine_y(bus, fine_y);
    }
}
