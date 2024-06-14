use crate::bus::types::*;

impl Bus {
    pub fn set_controller_a(&mut self, state : u8){
        self.controller_a.write_live(state);
    }
    pub fn set_controller_b(&mut self, state : u8){
        self.controller_b.write_live(state);
    }
    pub fn tick(&mut self){}
    pub fn frame(&mut self){}
    pub fn complete(&mut self){}
}
