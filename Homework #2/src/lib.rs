enum SwitchStatus {
    _On,
    _Off,
}

enum PowerUnit {
    _Watt(u32),
}

pub struct Rosette {
    _description: String,
    _status: SwitchStatus,
    _consumption: PowerUnit,
}
impl Rosette {
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

enum TemperatureUnit {
    _Celcium(f32),
}

pub struct Thermometer {
    _temperature: TemperatureUnit,
}
impl Thermometer {
    pub fn get_temperature(&self) -> f32 {
        todo!("Not implemented yet");
    }
}
