pub trait MapperT {
    fn cpu_r_mapt(&mut self, address : u16) -> u16;
    fn cpu_w_mapt(&mut self, address : u16, byte : u8) -> u16;
    fn ppu_r_mapt(&mut self, address : u16) -> u16;
    fn ppu_w_mapt(&mut self, address : u16, byte : u8) -> u16;
    fn clone_self(&self) -> Box<dyn MapperT>;
    fn reset(&mut self);
}


#[derive(Clone)] 
pub struct Mapper(pub Box<dyn MapperT>);

impl Mapper {
    pub fn cpu_r_map(&mut self, address : u16) -> u16{
        self.0.cpu_r_mapt(address)
    }
    pub fn cpu_w_map(&mut self, address : u16, byte : u8) -> u16{
        self.0.cpu_w_mapt(address, byte)
    }
    pub fn ppu_r_map(&mut self, address : u16) -> u16{
        self.0.ppu_r_mapt(address)
    }
    pub fn ppu_w_map(&mut self, address : u16, byte : u8) -> u16{
        self.0.ppu_w_mapt(address, byte)
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
