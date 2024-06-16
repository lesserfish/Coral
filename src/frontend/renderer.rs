use std::io;
use std::sync::Arc;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use super::shared;
use super::shared::{State, color_to_rgba, err};

// Types

struct Context<'a>{
    pub state : State,
    pub controller : u8,
    pub shared_data : Arc<shared::Data>,
    pub screen_texture: sdl2::render::Texture<'a>
}

fn create_context<'a>(shared_data : Arc<shared::Data>, creator : &'a TextureCreator<WindowContext>) -> io::Result<Context<'a>> {
    let state = State::Paused;
    let controller = 0;
    let screen_texture = creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 240).map_err(err)?;

    Ok(Context{state, controller, shared_data, screen_texture})
}

// Loop

fn send_command(ctx : &mut Context, command : shared::Command) -> io::Result<()>{
    let commands_lock = &ctx.shared_data.commands;
    commands_lock.write().map_err(err)?.push(command);

    Ok(())
}

fn toggle_pause(ctx : &mut Context) -> io::Result<()> {
    match ctx.state {
       State::Paused =>  {ctx.state = State::Running; send_command(ctx, shared::Command::Start)?;}
       State::Running => {ctx.state = State::Paused;  send_command(ctx, shared::Command::Stop)?;}
       _ => {}
    }

    Ok(())
}

fn handle_keydown(ctx : &mut Context, keycode : Keycode) -> io::Result<()>{
    match keycode {
        Keycode::Q         => {handle_exit(ctx)?;}
        Keycode::Space     => {toggle_pause(ctx)?;}
        Keycode::Right     => {ctx.controller |= 0x01}
        Keycode::Left      => {ctx.controller |= 0x02}
        Keycode::Down      => {ctx.controller |= 0x04}
        Keycode::Up        => {ctx.controller |= 0x08}
        Keycode::Return    => {ctx.controller |= 0x10}
        Keycode::Backspace => {ctx.controller |= 0x20}
        Keycode::Z         => {ctx.controller |= 0x40}
        Keycode::X         => {ctx.controller |= 0x80}
        _ => {}
    }
    
    Ok(())
}
fn handle_keyup(ctx : &mut Context, keycode : Keycode ){
    match keycode {
        Keycode::Right     => {ctx.controller &= !0x01}
        Keycode::Left      => {ctx.controller &= !0x02}
        Keycode::Down      => {ctx.controller &= !0x04}
        Keycode::Up        => {ctx.controller &= !0x08}
        Keycode::Return    => {ctx.controller &= !0x10}
        Keycode::Backspace => {ctx.controller &= !0x20}
        Keycode::Z         => {ctx.controller &= !0x40}
        Keycode::X         => {ctx.controller &= !0x80}
        _ => {}
    }
}

fn handle_exit(ctx : &mut Context) -> io::Result<()>{
    ctx.state = State::Exit;
    send_command(ctx, shared::Command::Exit)?;
    Ok(())
}


fn control(event_pump : &mut sdl2::EventPump, ctx : &mut Context) -> io::Result<()>{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..}  => { handle_exit(ctx)?; }
            Event::KeyDown {keycode: Some(k), ..} => {
                handle_keydown(ctx, k)?; 
            }
            Event::KeyUp { keycode: Some(k), ..} => {
                handle_keyup(ctx, k);
            }
            _ => {}
        }
    }
    Ok(())
}

fn update_screen(ctx: &mut Context) -> io::Result<()>{
    let screen_lock = &ctx.shared_data.screen;
    let screen = screen_lock.read().map_err(err)?.clone();
    ctx.screen_texture.with_lock(None, |texture_data, _pitch| {
        for x in 0..(240*256) {
            let address = x*4;
            let color = screen[x];
            let (r, g, b, a) = color_to_rgba(color);
            texture_data[address + 0] = a;
            texture_data[address + 1] = b;
            texture_data[address + 2] = g;
            texture_data[address + 3] = r;
        }
    }).map_err(err)?;

    Ok(())
}

fn update_controller(ctx : &mut Context) -> io::Result<()>{
    *ctx.shared_data.controller.write().map_err(err)? = ctx.controller;
    Ok(())
}

fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, ctx: &mut Context) -> io::Result<()>{
    canvas.clear();
    canvas.copy(&ctx.screen_texture, None, None).map_err(err)?;
    canvas.present();
    Ok(())
}


pub fn main(shared_data : Arc<shared::Data>)-> io::Result<()> {
    // Initialize SDL

    let sdl = sdl2::init().map_err(err)?;
    let video = sdl.video().map_err(err)?;
    let window = video.window("Coral", 256 * 3, 240 * 3).position_centered().build().map_err(err)?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(err)?;
    let creator = canvas.texture_creator();
    let mut event_pump = sdl.event_pump().unwrap();

    // Initialize Context

    let mut ctx = create_context(shared_data, &creator)?;

    // Main loop

    while ctx.state != State::Exit {
        control(&mut event_pump, &mut ctx)?;
        update_screen(&mut ctx)?;
        update_controller(&mut ctx)?;
        render(&mut canvas, &mut ctx)?;
    }

    Ok(())
}
