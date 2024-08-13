pub trait MapperT {
    fn cpu_read(&mut self, address : u16) -> u8;
    fn cpu_write(&mut self, address : u16, byte : u8);
    fn ppu_read(&mut self, address : u16) -> u8;
    fn ppu_write(&mut self, address : u16, byte : u8);
    fn clone_self(&self) -> Box<dyn MapperT>;
    fn reset(&mut self);
}


#[derive(Clone)] 
pub struct Mapper(pub Box<dyn MapperT>);

impl Mapper {
    pub fn cpu_read(&mut self, address : u16) -> u8{
        self.0.cpu_read(address)
    }
    pub fn cpu_write(&mut self, address : u16, byte : u8){
        self.0.cpu_write(address, byte)
    }
    pub fn ppu_read(&mut self, address : u16) -> u8{
        self.0.ppu_read(address)
    }
    pub fn ppu_write(&mut self, address : u16, byte : u8){
        self.0.ppu_write(address, byte)
    }
    pub fn reset(&mut self){
        self.0.reset();
    }
}

impl Clone for Box<dyn MapperT> {
    fn clone(&self) -> Self {
        self.clone_self()
    }
}
