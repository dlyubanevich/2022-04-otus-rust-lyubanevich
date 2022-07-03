use std::fmt::Display;

use crate::devices::traits::DeviceStatus;

pub struct Socket {
    description: String,
    switch_state: SwitchState,
    consumption: PowerUnit,
}
impl Socket {
    pub fn new(description: &str) -> Self {
        Socket {
            description: description.to_string(),
            switch_state: SwitchState::On,
            consumption: PowerUnit::Watt(0),
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
        match self.consumption {
            PowerUnit::Watt(value) => value,
        }
    }
}

impl DeviceStatus for Socket {
    fn get_status(&self) -> String {
        format!(
            "The socket is {}. Current consumption: {} watt",
            &self.switch_state,
            self.get_consumption()
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
enum PowerUnit {
    Watt(u32),
}
impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = &self.switch_state;
        let consumption = self.get_consumption();
        write!(f, "Socket {} ({} watt)", status, consumption)
    }
}
