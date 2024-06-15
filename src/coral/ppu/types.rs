pub trait Bus {
    fn read_byte(&mut self, address : u16) -> u8;
    fn write_byte(&mut self, address : u16, byte : u8);
    fn set_pixel(&mut self, position : (usize, usize), color : u8);
    fn trigger_nmi(&mut self);
    fn fetch_ppu(&mut self) -> &mut PPU;
}

#[derive(Copy, Clone, Debug)] 
pub struct Registers {
    pub control : u8,
    pub mask : u8,
    pub status : u8,
    pub fine_x : u8,
    pub data_buffer : u8,
    pub vram : u16,
    pub tram : u16,
    pub write_toggle : bool
}

#[derive(Copy, Clone, Debug)] 
pub struct Context {
    pub complete : bool,
    pub scanline : i32,
    pub cycle : i32,
    pub sprite_0_alpha : u8,
    pub sprite_0_x : i32,
    pub sprite_0_hit_position : i32,
    pub oam_address : u8
}

#[derive(Copy, Clone, Debug, PartialEq)] 
pub enum Priority {
    Front,
    Middle,
    Back,
    Unset
}

#[derive(Copy, Clone, Debug)] 
pub struct PixelInfo {
    pub color_index : u8,
    pub palette_index : u8,
    pub priority : Priority
}

#[derive(Copy, Clone, Debug)] 
pub struct PPU {
   pub registers : Registers,
   pub context : Context,
   pub oam_data : [u8; 0x100],
   pub fg_buffer : [PixelInfo; 32 * 8], // 256 pixels
   pub bg_buffer : [PixelInfo; 33 * 8], // 256 pixels + an additional 8 to accomodate scrolling
}

#[derive(Copy, Clone, Debug)] 
pub struct Sprite {
    pub id : usize,
    pub y_pos : u8,
    pub tile : u8,
    pub attribute : u8,
    pub x_pos : u8
}

#[derive(Copy, Clone, Debug)] 
pub enum ControlFlag {
    NametableX,
    NametableY,
    IncrementMode,
    PatternSprite,
    PatternBackground,
    SpriteSize,
    SlaveMode,
    EnableNMI
}

#[derive(Copy, Clone, Debug)] 
pub enum MaskFlag {
    Grayscale,
    RenderBackgroundLeft,
    RenderSpritesLeft,
    RenderBackground,
    RenderSprites,
    EnhancedRed,
    EnhanceGreen,
    EnhanceBlue
}

#[derive(Copy, Clone, Debug)] 
pub enum StatusFlag {
    SpriteOverflow,
    SpriteZeroHit,
    VerticalBlank
}

#[derive(Copy, Clone, Debug)] 
pub enum LoopyFlag {
    CoarseX,
    CoarseY,
    NametableX,
    NametableY,
    FineY
}

#[derive(Copy, Clone, Debug)] 
pub enum SpriteFlag {
    SpritePriority,
    SpriteHorizontalFlip,
    SpriteVerticalFlip
}


pub fn new() -> PPU {
    let registers = Registers{control: 0, mask: 0, status: 0, fine_x: 0, data_buffer: 0, vram: 0, tram: 0, write_toggle: false};
    let context = Context{complete: false, scanline: -1, cycle: 0, sprite_0_alpha: 0, sprite_0_x: -1, sprite_0_hit_position: -1, oam_address: 0};
    let oam_data = [0; 0x100];
    let fg_buffer = [PixelInfo{color_index: 0, palette_index: 0, priority: Priority::Unset} ; 32 * 8];
    let bg_buffer = [PixelInfo{color_index: 0, palette_index: 0, priority: Priority::Unset} ; 33 * 8];

    PPU {registers, context, oam_data, fg_buffer, bg_buffer}
}
