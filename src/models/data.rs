//! # Coinbase Advanced Data API
//!
//! `data` gives access to the Data API and the various endpoints associated with it.

use core::fmt;

use serde::{Deserialize, Serialize};

/// Various types of portfolios.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PortfolioType {
    /// Undefined portfolio type.
    Undefined,
    /// Default portfolio type.
    Default,
    /// Consumer portfolio type.
    Consumer,
    /// Intx portfolio type.
    Intx,
}

impl fmt::Display for PortfolioType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for PortfolioType {
    fn as_ref(&self) -> &str {
        match self {
            PortfolioType::Undefined => "UNDEFINED",
            PortfolioType::Default => "DEFAULT",
            PortfolioType::Consumer => "CONSUMER",
            PortfolioType::Intx => "INTX",
        }
    }
}

/// `KeyPermissions` represents the permissions associated with an API key.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeyPermissions {
    ///Indicates whether the API key has view permissions.
    pub can_view: bool,
    /// Indicates whether the API key has trade permissions.
    pub can_trade: bool,
    /// Indicates whether the API key has deposit/withdrawal permissions.
    pub can_transfer: bool,
    /// The portfolio ID associated with the API key.
    pub portfolio_uuid: String,
    /// The type of portfolio. Possible values: [UNDEFINED, DEFAULT, CONSUMER, INTX]
    pub portfolio_type: PortfolioType,
}
