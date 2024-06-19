use super::renderer;
use super::emulator;
use super::shared;
use std::thread;

pub fn main(filepath : String) -> std::io::Result<()>{
    let (s1, s2)= shared::new();

    let r = thread::spawn(move || {renderer::main(s1)});
    let e = thread::spawn(move || {emulator::main(filepath, s2)});

    e.join().unwrap()?;
    r.join().unwrap()?;

    Ok(())
}
