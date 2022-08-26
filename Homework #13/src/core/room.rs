pub struct Room {
    pub id: u32,
    pub name: String,
}

impl Room {
    pub fn new(id: u32, name: &str) -> Self {
        Room {
            id,
            name: name.to_string(),
        }
    }
}
