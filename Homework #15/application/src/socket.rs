use std::fmt::Display;

#[repr(C)]
pub struct Socket {
    switch_state: SwitchState,
    consumption: Watt,
}

impl Socket {
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

impl Default for Socket {
    fn default() -> Self {
        Self {
            switch_state: SwitchState::On,
            consumption: Watt::from(0),
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
        write!(
            f,
            "Socket {} ({}) Watt",
            &self.switch_state, &self.consumption
        )
    }
}
