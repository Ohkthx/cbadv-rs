//! # Coinbase Advanced Convert API
//!
//! `convert` gives access to the Convert API and the various endpoints associated with it.
//! This allows for the conversion between two currencies.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::errors::CbError;
use crate::traits::{Query, Request};
use crate::types::CbResult;
use crate::utils::QueryBuilder;

use super::shared::Balance;

/// Possible values for the trade status.
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum TradeStatus {
    /// Unspecified trade status.
    #[serde(rename = "TRADE_STATUS_UNSPECIFIED")]
    Unspecified,
    /// Trade has been created.
    #[serde(rename = "TRADE_STATUS_CREATED")]
    Created,
    /// Trade has started.
    #[serde(rename = "TRADE_STATUS_STARTED")]
    Started,
    /// Trade has been completed.
    #[serde(rename = "TRADE_STATUS_COMPLETED")]
    Completed,
    /// Trade has been canceled.
    #[serde(rename = "TRADE_STATUS_CANCELED")]
    Canceled,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trade {
    /// The trade id, used to get and commit the trade
    pub id: String,
    /// Possible values: [`TRADE_STATUS_UNSPECIFIED`, `TRADE_STATUS_CREATED`, `TRADE_STATUS_STARTED`, `TRADE_STATUS_COMPLETED`, `TRADE_STATUS_CANCELED`]
    pub status: TradeStatus,
    pub user_entered_amount: Balance,
    pub amount: Balance,
    pub subtotal: Balance,
    pub total: Balance,
    // List of fees associated with the trade
    pub fees: Vec<Fee>,
    pub total_fee: Fee,
    pub source: AccountDetail,
    pub target: AccountDetail,
    pub user_warnings: Vec<UserWarning>,
    pub user_reference: String,
    // The currency of the source account
    pub source_currency: String,
    // The currency of the target account
    pub target_currency: String,
    // The id of the source account
    pub source_id: String,
    // The id of the target account
    pub target_id: String,
    pub exchange_rate: Balance,
    // Tax details for the trade
    pub tax_details: Vec<TaxDetail>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Fee {
    pub title: String,
    pub description: String,
    pub amount: Balance,
    pub label: String,
    pub disclosure: Option<Disclosure>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Disclosure {
    pub title: String,
    pub description: String,
    pub link: Link,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountDetail {
    pub r#type: String,
    pub network: String,
    pub ledger_account: LedgerAccount,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerAccount {
    pub account_id: String,
    pub currency: String,
    pub owner: Owner,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub id: String,
    pub uuid: String,
    pub user_uuid: String,
    pub r#type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UnitPrice {
    pub target_to_fiat: PriceScale,
    pub target_to_source: PriceScale,
    pub source_to_fiat: PriceScale,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PriceScale {
    pub amount: Balance,
    pub scale: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserWarning {
    pub id: String,
    pub link: Link,
    pub context: WarningContext,
    pub code: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WarningContext {
    pub details: Vec<String>,
    pub title: String,
    pub link_text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CancellationReason {
    pub message: String,
    pub code: String,
    pub error_code: String,
    pub error_cta: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaxDetail {
    pub name: String,
    pub amount: Balance,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TradeIncentiveInfo {
    pub applied_incentive: bool,
    pub user_incentive_id: String,
    pub code_val: String,
    pub ends_at: String,
    pub fee_without_incentive: Balance,
    pub redeemed: bool,
}

/// Trade incentive to waive trade fees.
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeIncentiveMetadata {
    /// The user incentive id.
    pub user_incentive_id: Option<String>,
    /// A promo code for waiving fees.
    pub code_val: Option<String>,
}

/// Represents a request to create a convert quote.
#[serde_as]
#[derive(Serialize, Debug, Default)]
pub struct ConvertQuoteRequest {
    /// The currency of the account to convert from, e.g. USD
    pub from_account: String,
    /// The currency of the account to convert to, e.g. USDC
    pub to_account: String,
    /// The amount to convert in the currency of the from_account.
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    /// Trade incentive to waive trade fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_incentive_metadata: Option<TradeIncentiveMetadata>,
}

impl Request for ConvertQuoteRequest {
    fn check(&self) -> CbResult<()> {
        if self.from_account.is_empty() {
            return Err(CbError::BadRequest("from_account is required".to_string()));
        } else if self.to_account.is_empty() {
            return Err(CbError::BadRequest("to_account is required".to_string()));
        } else if self.amount <= 0.0 {
            return Err(CbError::BadRequest(
                "amount must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl ConvertQuoteRequest {
    /// Creates a new instance of the `ConvertQuoteRequest`.
    ///
    /// # Arguments
    ///
    /// * `from_account` - The currency of the account to convert from, e.g. USD
    /// * `to_account` - The currency of the account to convert to, e.g. USDC
    /// * `amount` - The amount to convert in the currency of the `from_account`.
    pub fn new(from_account: &str, to_account: &str, amount: f64) -> Self {
        Self {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
            amount,
            trade_incentive_metadata: None,
        }
    }

    /// Sets the trade incentive to waive trade fees.
    pub fn trade_incentive_metadata(mut self, metadata: TradeIncentiveMetadata) -> Self {
        self.trade_incentive_metadata = Some(metadata);
        self
    }
}

/// Represents parameters to obtain a currency conversion.
///
/// # Required Parameters
///
/// * `from_account` - The currency of the account to convert from, e.g. USD
/// * `to_account` - The currency of the account to convert to, e.g. USDC
#[derive(Serialize, Default, Debug)]
pub struct ConvertQuery {
    /// Originating account.
    pub from_account: String,
    /// Sending account.
    pub to_account: String,
}
impl Request for ConvertQuery {
    fn check(&self) -> CbResult<()> {
        if self.from_account.is_empty() {
            return Err(CbError::BadRequest("from_account is required".to_string()));
        } else if self.to_account.is_empty() {
            return Err(CbError::BadRequest("to_account is required".to_string()));
        }
        Ok(())
    }
}

impl Query for ConvertQuery {
    fn check(&self) -> CbResult<()> {
        if self.from_account.is_empty() {
            return Err(CbError::BadQuery("from_account is required".to_string()));
        } else if self.to_account.is_empty() {
            return Err(CbError::BadQuery("to_account is required".to_string()));
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("from_account", &self.from_account)
            .push("to_account", &self.to_account)
            .build()
    }
}

impl ConvertQuery {
    /// Creates a new instance of the `ConvertQuery`.
    ///
    /// # Arguments
    ///
    /// * `from_account` - The currency of the account to convert from, e.g. USD
    /// * `to_account` - The currency of the account to convert to, e.g. USDC
    pub fn new(from_account: &str, to_account: &str) -> Self {
        Self {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
        }
    }

    /// Sets the originating account.
    /// Note: This is a required field.
    pub fn from_account(mut self, from_account: &str) -> Self {
        self.from_account = from_account.to_string();
        self
    }

    /// Sets the sending account.
    /// Note: This is a required field.
    pub fn to_account(mut self, to_account: &str) -> Self {
        self.to_account = to_account.to_string();
        self
    }
}

/// Response from the convert API endpoint.
#[derive(Deserialize, Debug)]
pub(crate) struct TradeWrapper {
    pub(crate) trade: Trade,
}

impl From<TradeWrapper> for Trade {
    fn from(wrapper: TradeWrapper) -> Self {
        wrapper.trade
    }
}
