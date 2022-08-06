use smart_home::devices::{Socket, Thermometer};

fn main() {
    let socket = Socket::new("Huawei");
    let thermometer = Thermometer::new("Apple");

    let storage = Storage;
    println!("{}", socket.get_data(&storage));
    println!("{}", thermometer.get_data(&storage));
}

struct Storage;

impl Visitor for Storage {
    type Result = String;

    fn visit_socket(&self, socket: &Socket) -> Self::Result {
        format!(
            "Socket [{}] current consumption: {} watt",
            socket.get_description(),
            socket.get_consumption()
        )
    }

    fn visit_thermometer(&self, thermometer: &Thermometer) -> Self::Result {
        format!(
            "Thermometer [{}] current temperature: {}",
            thermometer.get_description(),
            thermometer.get_temperature()
        )
    }
}

trait Visitor {
    type Result;

    fn visit_socket(&self, socket: &Socket) -> Self::Result;
    fn visit_thermometer(&self, thermometer: &Thermometer) -> Self::Result;
}

trait Data {
    fn get_data<V: Visitor>(&self, visitor: &V) -> V::Result;
}

impl Data for Socket {
    fn get_data<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_socket(self)
    }
}

impl Data for Thermometer {
    fn get_data<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_thermometer(self)
    }
}
