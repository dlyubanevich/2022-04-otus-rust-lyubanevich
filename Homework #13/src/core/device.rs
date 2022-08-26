pub struct Device {
    pub id: u32,
    pub name: String,
}
impl Device {
    pub fn new(id: u32, name: &str) -> Self {
        Device {
            id,
            name: name.to_string(),
        }
    }
}
