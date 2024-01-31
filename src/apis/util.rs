//! # Coinbase Advanced Utils API
//!
//! `util` gives access to the Utils API and the various endpoints associated with it.
//! Some of the features include getting the API current time in ISO format.

use crate::constants::utils::UNIXTIME_ENDPOINT;
use crate::errors::CbAdvError;
use crate::signer::Signer;
use crate::traits::NoQuery;
use crate::types::CbResult;
use crate::util::UnixTime;

/// Provides access to the Utils API for the service.
pub struct UtilApi {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl UtilApi {
    /// Creates a new instance of the Utils API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Get the current time from the Coinbase Advanced API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/time
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getunixtime
    pub async fn unixtime(&mut self) -> CbResult<UnixTime> {
        match self.signer.get(UNIXTIME_ENDPOINT, &NoQuery).await {
            Ok(value) => match value.json::<UnixTime>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("util unixtime object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
