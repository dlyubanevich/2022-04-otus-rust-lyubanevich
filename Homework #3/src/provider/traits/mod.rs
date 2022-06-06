pub trait InfoProvider {
    fn get_device_report(&self, room: &str, device: &str) -> String;
}
