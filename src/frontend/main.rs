use super::renderer;
use super::emulator;
use super::shared;
use std::thread;

pub fn main(filepath : String){
    let (s1, s2)= shared::new();

    let r = thread::spawn(move || {renderer::main(s1)});
    let e = thread::spawn(move || {emulator::main(filepath, s2)});


    let _ = r.join().unwrap();
    let _ = e.join().unwrap();
}
