use crate::devices::traits::DeviceStatus;

enum TemperatureUnit {
    Celcium(f32),
}

pub struct Thermometer {
    _description: String,
    temperature: TemperatureUnit,
}
impl Thermometer {
    pub fn new(description: &str) -> Self {
        Thermometer {
            _description: description.to_string(),
            temperature: TemperatureUnit::Celcium(0_f32),
        }
    }
    pub fn get_temperature(&self) -> f32 {
        match self.temperature {
            TemperatureUnit::Celcium(value) => value,
        }
    }
}
impl DeviceStatus for Thermometer {
    fn get_status(&self) -> String {
        let status = self.get_temperature();
        format!("Thermometer shows {status} degrees")
    }
}
