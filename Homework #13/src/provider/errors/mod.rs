use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderErrors {
    #[error("There is already added device with owner: [{0}] and name: [{1}]")]
    NotUniqueDeviceByOwner(String, String),
}
