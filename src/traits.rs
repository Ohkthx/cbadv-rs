//! Traits that extend features.

/// Generic configuration file with the minimum requirements for API access. This is used to implement on custom configurations and to be passed when creating REST and WebSocket clients.
pub trait ConfigFile {
    /// API Key provided by the service.
    fn cb_api_key(&self) -> &str;
    /// API Secret provided by the service.
    fn cb_api_secret(&self) -> &str;
}
