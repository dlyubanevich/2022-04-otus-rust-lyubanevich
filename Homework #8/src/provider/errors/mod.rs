#[derive(Debug)]
pub enum ProviderErrors {
    NotUniqueDeviceByOwner(String, String),
}
impl std::fmt::Display for ProviderErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderErrors::NotUniqueDeviceByOwner(owner, device_name) => {
                write!(
                    f,
                    "There is already added device with owner: [{}] and name: [{}]",
                    owner, device_name
                )
            }
        }
    }
}
impl std::error::Error for ProviderErrors {}
