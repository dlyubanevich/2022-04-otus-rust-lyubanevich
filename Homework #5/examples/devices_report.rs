use smart_home::core::SmartHome;
use smart_home::devices::Socket;
use smart_home::provider::{DeviceInfoProvider, DeviceType};

fn main() {
    let room_name = "Bedroom";
    let device_socket_name = "Socket";
    let device_thermometer_name = "Thermometer";

    let mut smart_home = SmartHome::new("My home");
    smart_home.add_room(room_name);
    smart_home.add_device(room_name, device_socket_name);
    smart_home.add_device(room_name, device_thermometer_name);

    let socket = Socket::new(device_socket_name);

    let mut info_provider = DeviceInfoProvider::default();
    match info_provider.add_device(room_name, device_socket_name, DeviceType::Socket(socket)) {
        Err(e) => println!("{}", e),
        _ => (),
    };

    let report = smart_home.create_report(&info_provider);
    println!("Report #1: {report}");
}
