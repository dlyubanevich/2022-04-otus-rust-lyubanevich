use std::fmt::Display;

pub struct Thermometer {
    description: String,
    temperature: Celcium,
}
impl Thermometer {
    pub fn new(description: &str) -> Self {
        Thermometer {
            description: description.to_string(),
            temperature: Celcium::from(0_f32),
        }
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature.0
    }
    pub fn set_temperature(&mut self, value: f32) {
        self.temperature = Celcium::from(value);
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

struct Celcium(f32);
impl From<f32> for Celcium {
    fn from(value: f32) -> Self {
        Celcium(value)
    }
}
