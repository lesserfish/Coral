pub trait Bus {
    fn read_byte(&mut self, address : u16) -> u8;
    fn write_byte(&mut self, address : u16, byte : u8);
    fn peek_byte(&self, address : u16) -> u8;
    fn set_pixel(&mut self, position : (u16, u16), color : u8);
    fn trigger_nmi(&mut self);
    fn fetch_ppu(&mut self) -> &mut PPU;
}

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

pub struct Context {
    pub complete : bool,
    pub scanline : i32,
    pub cycle : i32,
    pub sprite_0_alpha : u8,
    pub sprite_0_x : i32,
    pub sprite_0_hit_position : i32
}
pub struct PPU {
   pub registers : Registers,
   pub context : Context,
   pub oam_data : [u8; 0xFF],
   pub bg_buffer : [u8; 32 * 9],
   pub fg_buffer : [u8; 32 * 8]
}

pub struct Sprite {
    pub id : u8,
    pub y_pos : u8,
    pub tile : u8,
    pub attribute : u8,
    pub x_pos : u8
}

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

pub enum StatusFlag {
    SpriteOverflow,
    SpriteZeroHit,
    VerticalBlank
}

pub enum LoopyFlag {
    CoarseX,
    CoarseY,
    NametableX,
    NametableY,
    FineY
}

pub enum SpriteFlag {
    SpritePalette,
    SpritePriority,
    SpriteHorizontalFlip,
    SpriteVerticalFlip
}

