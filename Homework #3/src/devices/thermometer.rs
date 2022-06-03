use crate::devices::traits::DeviceStatus;

enum TemperatureUnit {
    Celcium(f32),
}

pub struct Thermometer {
    description: String,
    temperature: TemperatureUnit,
}
impl Thermometer {
    pub fn new(description: &str) -> Self {
        Thermometer{
            description: description.to_string(),
            temperature: TemperatureUnit::Celcium(0_f32)    
        }    
    }
    pub fn get_temperature(&self) -> f32 {
        todo!("Not implemented yet");
    }
}
impl DeviceStatus for Thermometer {
    fn get_status(&self) -> &str {
        self.description.as_str()
    }
}