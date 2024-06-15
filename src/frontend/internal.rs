use std::io;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::coral::bus;
use crate::frontend::utils;

use super::utils::color_to_rgba;

// Types
pub fn err<T : std::string::ToString>(e : T) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

pub struct Context{
    pub running : bool,
    pub nes : bus::types::Bus
}

pub struct Textures<'a> {
    pub pattern : sdl2::render::Texture<'a>
}

// Initializers

pub fn create_textures(creator : &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> io::Result<Textures> {
    let t= creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 240).map_err(err)?;
    let textures = Textures{pattern: t};

    Ok(textures)
}

pub fn create_context() -> io::Result<Context>{
    let n = bus::load("/home/lesserfish/Documents/Code/Shrimp/Tools/Roms/Super Mario Bros. (World).nes")?;
    let ctx = Context{running: true, nes : n};

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

pub fn update_pattern(ctx: &mut Context, texture_data : &mut [u8], pitch : usize){
    for y in 0..240 {
        for x in 0..256 {
            let address = y * pitch + x*4;
            let color = ctx.nes.get_pixel(x, y);
            let (r, g, b, a) = color_to_rgba(color);
            texture_data[address + 0] = a;
            texture_data[address + 1] = b;
            texture_data[address + 2] = g;
            texture_data[address + 3] = r;
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
