use smart_home::devices::{Socket, Thermometer};

fn main() {
    let mut storage = Storage::new();
    storage.add_device(Box::new(Socket::new("Huawei")));
    storage.add_device(Box::new(Thermometer::new("Apple")));

    storage.info();
}

struct Storage {
    devices: Vec<Box<dyn Data>>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            devices: Vec::new(),
        }
    }
    fn add_device(&mut self, device: Box<dyn Data>) {
        self.devices.push(device);
    }
    fn info(&self) {
        self.devices.iter().for_each(|device| {
            let result = self.visit(device.as_ref());
            println!("{}", result);
        });
    }
}
trait Data {
    fn get_data(&self) -> String;
}

impl Data for Socket {
    fn get_data(&self) -> String {
        format!(
            "Socket [{}] current consumption: {} watt",
            self.get_description(),
            self.get_consumption()
        )
    }
}
impl Data for Thermometer {
    fn get_data(&self) -> String {
        format!(
            "Thermometer [{}] current temperature: {}",
            self.get_description(),
            self.get_temperature()
        )
    }
}

trait Visitor {
    type Result;

    fn visit(&self, data: &dyn Data) -> Self::Result;
}

impl Visitor for Storage {
    type Result = String;

    fn visit(&self, data: &dyn Data) -> Self::Result {
        data.get_data()
    }
}
