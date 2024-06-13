use std::io;
use crate::frontend::internal::*;

pub fn main()-> io::Result<()> {
    //
    // Initialize SDL

    let sdl = sdl2::init().map_err(err)?;
    let video = sdl.video().map_err(err)?;
    let window = video.window("Coral", 256 * 4, 128 * 4).position_centered().build().map_err(err)?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(err)?;
    let creator = canvas.texture_creator();
    let mut event_pump = sdl.event_pump().unwrap();

    // Initialize Context

    let mut ctx = create_context()?;
    let mut textures = create_textures(&creator)?;

    while ctx.running {
        control(&mut event_pump, &mut ctx)?;
        update_textures(&mut textures, &mut ctx)?;
        render(&mut canvas, &mut textures)?;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
