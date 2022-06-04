use smart_home::core::{Room, SmartHome};
use smart_home::devices::Socket;
use smart_home::provider::{DeviceInfoProvider, DeviceType};

fn main() {
    let room_name = "Bedroom";
    let device_socket_name = "Socket";
    let device_thermometer_name = "Thermometer";

    let mut bedroom = Room::new(room_name);
    bedroom.add_device(device_socket_name);
    bedroom.add_device(device_thermometer_name);

    let mut smart_home = SmartHome::new("My home");
    smart_home.add_room(bedroom);

    let mut socket = Socket::new(device_socket_name);
    socket.switch_off();

    let mut info_provider = DeviceInfoProvider::default();
    info_provider.add_device(room_name, device_socket_name, DeviceType::Socket(&socket));

    let report = smart_home.create_report(info_provider);
    println!("Report #1: {report}");
}
