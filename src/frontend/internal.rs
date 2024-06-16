use std::io;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::coral::bus;

use super::utils::color_to_rgba;

// Types
pub fn err<T : std::string::ToString>(e : T) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

pub struct Context{
    pub running : bool,
    pub nes : bus::types::Bus,
    pub screen : [u8; 256 * 240],
    pub controller : u8
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

pub fn create_context(filepath : String) -> io::Result<Context>{
    let n = bus::load(filepath)?;
    let ctx = Context{running: true, nes : n, screen: [0; 256 * 240], controller: 0};

    Ok(ctx)
}

// Loop

pub fn handle_keydown(ctx : &mut Context, keycode : Keycode){
    match keycode {
        Keycode::Q => {ctx.running = false}
        Keycode::Z => {ctx.controller |= 0x40}
        Keycode::X => {ctx.controller |= 0x80}
        Keycode::Up => {ctx.controller |= 0x08}
        Keycode::Down => {ctx.controller |= 0x04}
        Keycode::Left => {ctx.controller |= 0x02}
        Keycode::Right => {ctx.controller |= 0x01}
        Keycode::Return => {ctx.controller |= 0x10}
        Keycode::Backspace => {ctx.controller |= 0x20}
        _ => {}
    }
}
pub fn handle_keyup(ctx : &mut Context, keycode : Keycode ){
    match keycode {
        Keycode::Z => {ctx.controller &= !0x40}
        Keycode::X => {ctx.controller &= !0x80}
        Keycode::Up => {ctx.controller &= !0x08}
        Keycode::Down => {ctx.controller &= !0x04}
        Keycode::Left => {ctx.controller &= !0x02}
        Keycode::Right => {ctx.controller &= !0x01}
        Keycode::Return => {ctx.controller &= !0x10}
        Keycode::Backspace => {ctx.controller &= !0x20}
        _ => {}
    }
}

pub fn control(event_pump : &mut sdl2::EventPump, ctx : &mut Context) -> io::Result<()>{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..}  => {ctx.running = false;}
            Event::KeyDown {keycode: Some(k), ..} => {
               handle_keydown(ctx, k); 
            }
            Event::KeyUp { keycode: Some(k), ..} => {
                handle_keyup(ctx, k);
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn update_texture(ctx: &mut Context, texture_data : &mut [u8], _pitch : usize){
    ctx.nes.frame();
    ctx.nes.copy_to_screen(&mut ctx.screen);
    ctx.nes.set_controller_a(ctx.controller);
    for x in 0..(240*256) {
        let address = x*4;
        let color = ctx.screen[x];
        let (r, g, b, a) = color_to_rgba(color);
        texture_data[address + 0] = a;
        texture_data[address + 1] = b;
        texture_data[address + 2] = g;
        texture_data[address + 3] = r;
    }
}
pub fn update_textures(textures : &mut Textures, ctx : &mut Context) -> io::Result<()>{
    textures.pattern.with_lock(None, |d, p| update_texture(ctx, d, p)).map_err(err)?;
    Ok(())
}

pub fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, textures : &mut Textures) -> io::Result<()>{
    canvas.clear();
    canvas.copy(&textures.pattern, None, None).map_err(err)?;
    canvas.present();
    Ok(())
}
