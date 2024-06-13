use std::io;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::coral::cartridge;

// Types
pub fn err<T : std::string::ToString>(e : T) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

pub struct Context{
    pub running : bool,
    pub cart : cartridge::Cartridge
}

pub struct Textures<'a> {
    pub pattern : sdl2::render::Texture<'a>
}

// Initializers

pub fn create_textures(creator : &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> io::Result<Textures> {
    let t= creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 128).map_err(err)?;
    let textures = Textures{pattern: t};

    Ok(textures)
}

pub fn create_context() -> io::Result<Context>{
    let cartridge = cartridge::load_cartridge("/home/lesserfish/Documents/Code/Shrimp/Tools/Roms/Super Mario Bros. (World).nes".to_string())?;
    let ctx = Context{running: true, cart : cartridge};

    Ok(ctx)
}

// Loop

pub fn control(event_pump : &mut sdl2::EventPump, ctx : &mut Context) -> io::Result<()>{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    ctx.running = false;
                },
            _ => {}
        }
    }
    Ok(())
}

fn color(pixel : u8) -> (u8, u8, u8, u8){
    match pixel {
        0 => (255, 255, 255, 255),
        1 => (122, 122, 122, 122),
        2 => (80, 80, 80, 80),
        _ => (0, 0, 0, 0),
    }
}
pub fn update_pattern(ctx: &mut Context, texture_data : &mut [u8], pitch : usize){
    for pt in 0..2 {
        for x in 0..16 {
            for y in 0..16 {
                let tile_offset = 0x1000*pt + 256 * y + 16 * x;
                for line in 0..8 {
                    let lsb = ctx.cart.chr_data[tile_offset + line];
                    let msb = ctx.cart.chr_data[tile_offset + line + 8];

                    for pixel in 0..8 {
                        let px = x * 8 + (7 - pixel) + 128 * pt;
                        let py = y * 8 + line;

                        let pl = (lsb >> pixel) & 0x01;
                        let ph = (msb >> pixel) & 0x01;

                        let pc = (ph << 1) + pl;
                        let (r, g, b, a) = color(pc);

                        let address = py * pitch + px * 4;

                        texture_data[address + 0] = r;
                        texture_data[address + 1] = g;
                        texture_data[address + 2] = b;
                        texture_data[address + 3] = a;
                    }
                }
            }
        }
    }
}
pub fn update_textures(textures : &mut Textures, ctx : &mut Context) -> io::Result<()>{
    textures.pattern.with_lock(None, |d, p| update_pattern(ctx, d, p)).map_err(err)?;
    Ok(())
}

pub fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, textures : &mut Textures) -> io::Result<()>{
    canvas.clear();
    canvas.copy(&textures.pattern, None, None).map_err(err)?;
    canvas.present();
    Ok(())
}
