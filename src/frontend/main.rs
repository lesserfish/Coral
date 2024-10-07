use super::renderer;
use super::emulator;
use super::shared;
use std::thread;

pub fn main(filepath : String) -> std::io::Result<()>{
    let (s1, s2)= shared::new();

    let e = thread::spawn(move || {emulator::main(filepath, s2)});
    renderer::main(s1).unwrap();
    e.join().unwrap()?;
    Ok(())
}
