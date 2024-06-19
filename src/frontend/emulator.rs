use std::io;
use std::sync::Arc;
use super::shared;
use super::shared::{State, err};
use coral::bus;

struct Context {
    nes : bus::Bus,
    shared_data : Arc<shared::Data>,
    state : State
}

fn create_context(filepath : String, shared_data : Arc<shared::Data>) -> io::Result<Context> {
    let nes = bus::load(filepath)?;
    let state = State::Running;

    Ok(Context{nes, shared_data, state})
}

fn handle_command(ctx : &mut Context, command : shared::Command){
   match command {
        shared::Command::Stop => {ctx.state = State::Paused}
        shared::Command::Start => {ctx.state = State::Running}
        shared::Command::Exit => {ctx.state = State::Exit}
   } 
}

fn handle_commands(ctx : &mut Context) -> io::Result<()>{
    let commands = ctx.shared_data.commands.read().map_err(err)?.clone();
    ctx.shared_data.commands.write().map_err(err)?.clear();

    for command in commands {
        handle_command(ctx, command);
    }

    Ok(())
}

fn update_controller(ctx : &mut Context) -> io::Result<()> {
    let controller = *ctx.shared_data.controller.read().map_err(err)?;
    ctx.nes.set_controller_a(controller);
    Ok(())
}

fn save_screen(ctx : &mut Context) -> io::Result<()>{
    let screen = &mut *ctx.shared_data.screen.write().map_err(err)?;
    ctx.nes.copy_to_screen(screen);
    Ok(())
}

pub fn main(filepath : String, shared_data : Arc<shared::Data>) -> io::Result<()>{
    let mut ctx = create_context(filepath, shared_data)?;

    while ctx.state != State::Exit {
        handle_commands(&mut ctx)?;
        update_controller(&mut ctx)?;
        if ctx.state == State::Running {
            let time= std::time::Instant::now();
            ctx.nes.frame();
            save_screen(&mut ctx)?;
            let ellapsed_time = time.elapsed();
            let sleep_duration = std::time::Duration::from_micros(16000) - ellapsed_time;
            std::thread::sleep(sleep_duration);
        }
    }

    Ok(())
}
