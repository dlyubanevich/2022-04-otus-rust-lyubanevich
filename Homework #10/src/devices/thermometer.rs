use std::fmt::Display;

pub struct Thermometer {
    description: String,
    temperature: TemperatureUnit,
}
impl Thermometer {
    pub fn new(description: &str) -> Self {
        Thermometer {
            description: description.to_string(),
            temperature: TemperatureUnit::Celcium(0_f32),
        }
    }
    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn get_temperature(&self) -> f32 {
        match self.temperature {
            TemperatureUnit::Celcium(value) => value,
        }
    }
    pub fn set_temperature(&mut self, value: f32) {
        self.temperature = TemperatureUnit::Celcium(value);
    }
}
impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = self.get_temperature();
        write!(
            f,
            "Thermometer [{}] shows {} degrees",
            self.description, status
        )
    }
}

enum TemperatureUnit {
    Celcium(f32),
}
