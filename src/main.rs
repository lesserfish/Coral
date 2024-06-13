use coral::cartridge;

fn main() {
    let cart = cartridge::load_cartridge("/home/lesserfish/Documents/Code/Shrimp/Tools/Roms/Super_mario_brothers.nes".to_string()).unwrap();
    println!("{:?}", cart.header);
}
