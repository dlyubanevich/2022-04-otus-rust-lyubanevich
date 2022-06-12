use std::fmt::Display;

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
    pub fn get_consumption(&self) -> u32 {
        match self.consumption {
            PowerUnit::Watt(value) => value,
        }
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
