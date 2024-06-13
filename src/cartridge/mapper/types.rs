pub trait MapperT {
    fn cpu_r_mapt(&mut self, address : u16) -> u16;
    fn cpu_w_mapt(&mut self, address : u16) -> u16;
    fn cpu_p_mapt(&mut self, address : u16) -> u16;
    fn ppu_r_mapt(&mut self, address : u16) -> u16;
    fn ppu_w_mapt(&mut self, address : u16) -> u16;
    fn ppu_p_mapt(&mut self, address : u16) -> u16;
}


pub struct Mapper(pub Box<dyn MapperT>);

impl Mapper {
    fn cpu_r_map(&mut self, address : u16) -> u16{
        self.0.cpu_r_mapt(address)
    }
    fn cpu_w_map(&mut self, address : u16) -> u16{
        self.0.cpu_w_mapt(address)
    }
    fn cpu_p_map(&mut self, address : u16) -> u16{
        self.0.cpu_p_mapt(address)
    }
    fn ppu_r_map(&mut self, address : u16) -> u16{
        self.0.ppu_r_mapt(address)
    }
    fn ppu_w_map(&mut self, address : u16) -> u16{
        self.0.ppu_w_mapt(address)
    }
    fn ppu_p_map(&mut self, address : u16) -> u16{
        self.0.ppu_p_mapt(address)
    }
}
