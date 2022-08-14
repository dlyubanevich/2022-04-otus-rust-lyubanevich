use std::fmt::Display;

use crate::devices::traits::DeviceStatus;

pub struct Socket {
    description: String,
    switch_state: SwitchState,
    consumption: Watt,
}
impl Socket {
    pub fn new(description: &str) -> Self {
        Socket {
            description: description.to_string(),
            switch_state: SwitchState::On,
            consumption: Watt::from(0),
        }
    }
    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn switch_on(&mut self) {
        self.switch_state = SwitchState::On;
    }
    pub fn switch_off(&mut self) {
        self.switch_state = SwitchState::Off;
    }
    pub fn is_on(&self) -> bool {
        match self.switch_state {
            SwitchState::On => true,
            SwitchState::Off => false,
        }
    }
    pub fn get_consumption(&self) -> u32 {
        self.consumption.0
    }
}

impl DeviceStatus for Socket {
    fn get_status(&self) -> String {
        format!(
            "The socket is {}. Current consumption: {}",
            &self.switch_state, &self.consumption
        )
    }
}

#[derive(Debug)]
enum SwitchState {
    On,
    Off,
}
impl Display for SwitchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
struct Watt(u32);
impl From<u32> for Watt {
    fn from(value: u32) -> Self {
        Watt(value)
    }
}
impl Display for Watt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Watt", self.0)
    }
}
impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Socket {} ({})", &self.switch_state, &self.consumption)
    }
}
