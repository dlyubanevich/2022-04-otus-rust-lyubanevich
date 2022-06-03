use crate::devices::traits::DeviceStatus;

enum SwitchState {
    On,
    Off,
}

enum PowerUnit {
    Watt(u32),
}

pub struct Socket {
    description: String,
    status: SwitchState,
    consumption: PowerUnit,
}
impl Socket {
    pub fn new(description: &str) -> Self {
        Socket{
            description: description.to_string(),
            status: SwitchState::On,
            consumption: PowerUnit::Watt(0)
        }  
    }
    pub fn get_description(&self) -> &str {
        todo!("Not implemented yet");
    }
    pub fn switch_on(&mut self) {
        todo!("Not implemented yet");
    }
    pub fn switch_off(&mut self) {
        todo!("Not implemented yet");
    }
    pub fn get_consumption(&self) -> u32 {
        todo!("Not implemented yet");
    }
}
impl DeviceStatus for Socket {
    fn get_status(&self) -> &str {
        self.description.as_str()
    }
}
