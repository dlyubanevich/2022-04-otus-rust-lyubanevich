use smart_home::{core::SmartHome, provider::InfoProvider};

struct Provider {}
impl InfoProvider for Provider {
    fn get_device_report(&self, room: &str, device: &str) -> String {
        format!("Room: [{room}], Device: [{device}]")
    }
}

#[test]
fn should_create_report() {
    let home_name = "My home";
    let device_name = "Socket";
    let room_name = "Bedroom";

    let mut smart_home = SmartHome::new(home_name);
    smart_home.add_device(room_name, device_name);
    let result = smart_home.create_report(&Provider {});
    let expected =
        format!("Smart home [{home_name}]:\nRoom: [{room_name}], Device: [{device_name}]");
    assert_eq!(result, expected)
}
