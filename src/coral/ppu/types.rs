pub trait Bus {
    fn read_byte(&mut self, address : u16) -> u8;
    fn write_byte(&mut self, address : u16, byte : u8);
    fn peek_byte(&self, address : u16) -> u8;
    fn set_pixel(&mut self, position : (u16, u16), color : u8);
    fn trigger_nmi(&mut self);
    fn fetch_ppu(&mut self) -> &mut PPU;
    fn get_ppu(&mut self) -> PPU {
        *self.fetch_ppu()
    }
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

#[derive(Copy, Clone, Debug)] 
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
   pub oam_data : [u8; 0xFF],
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

