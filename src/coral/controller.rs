pub struct Controller {
    pub state_data : u8,
    pub live_data : u8
}

impl Controller{
    pub fn reset(&mut self){
        self.state_data = 0;
        self.live_data = 0;
    }
    pub fn read(&mut self) -> u8 {
       let state = self.state_data >> 7; 
       self.state_data = self.state_data << 1;
       state
    }
    pub fn write(&mut self){
        self.state_data = self.live_data;
    }
    pub fn write_live(&mut self, state : u8){
        self.live_data = state;
    }

}

pub fn new() -> Controller {
    Controller { state_data: 0, live_data: 0 }
}
