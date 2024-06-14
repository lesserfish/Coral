pub trait MapperT {
    fn cpu_r_mapt(&mut self, address : u16) -> u16;
    fn cpu_w_mapt(&mut self, address : u16) -> u16;
    fn cpu_p_mapt(&mut self, address : u16) -> u16;
    fn ppu_r_mapt(&mut self, address : u16) -> u16;
    fn ppu_w_mapt(&mut self, address : u16) -> u16;
    fn ppu_p_mapt(&mut self, address : u16) -> u16;
}


pub struct Mapper(pub Box<dyn MapperT>);


