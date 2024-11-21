//! # Coinbase Advanced Convert API
//!
//! `convert` gives access to the Convert API and the various endpoints associated with it.
//! This allows for the conversion between two currencies.

use serde::{Deserialize, Serialize};

use crate::{traits::Query, utils::QueryBuilder};

use super::shared::Balance;

#[derive(Deserialize, Debug)]
pub struct Trade {
    /// The trade id, used to get and commit the trade
    pub id: String,
    /// Possible values: [TRADE_STATUS_UNSPECIFIED, TRADE_STATUS_CREATED, TRADE_STATUS_STARTED, TRADE_STATUS_COMPLETED, TRADE_STATUS_CANCELED]
    pub status: String,
    pub user_entered_amount: Balance,
    pub amount: Balance,
    pub subtotal: Balance,
    pub total: Balance,
    /// List of fees associated with the trade
    pub fees: Vec<Fee>,
    pub total_fee: FeeDetail,
    pub source: AccountDetail,
    pub target: AccountDetail,
    pub unit_price: UnitPrice,
    pub user_warnings: Vec<UserWarning>,
    pub user_reference: String,
    /// The currency of the source account
    pub source_currency: String,
    /// The currency of the target account
    pub target_currency: String,
    pub cancellation_reason: CancellationReason,
    /// The id of the source account
    pub source_id: String,
    /// The id of the target account
    pub target_id: String,
    pub exchange_rate: Balance,
    /// Tax details for the trade
    pub tax_details: Vec<TaxDetail>,
    pub trade_incentive_info: TradeIncentiveInfo,
    pub total_fee_without_tax: FeeDetail,
    pub fiat_denoted_total: Balance,
}

#[derive(Deserialize, Debug)]
pub struct Fee {
    pub title: String,
    pub description: String,
    pub amount: Balance,
    pub label: String,
    pub disclosure: Disclosure,
}

#[derive(Deserialize, Debug)]
pub struct Disclosure {
    pub title: String,
    pub description: String,
    pub link: Link,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct FeeDetail {
    pub title: String,
    pub description: String,
    pub amount: Balance,
    pub label: String,
    pub disclosure: Disclosure,
}

#[derive(Deserialize, Debug)]
pub struct AccountDetail {
    pub r#type: String,
    pub network: String,
    pub ledger_account: LedgerAccount,
}

#[derive(Deserialize, Debug)]
pub struct LedgerAccount {
    pub account_id: String,
    pub currency: String,
    pub owner: Owner,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub id: String,
    pub uuid: String,
    pub user_uuid: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct UnitPrice {
    pub target_to_fiat: PriceScale,
    pub target_to_source: PriceScale,
    pub source_to_fiat: PriceScale,
}

#[derive(Deserialize, Debug)]
pub struct PriceScale {
    pub amount: Balance,
    pub scale: i32,
}

#[derive(Deserialize, Debug)]
pub struct UserWarning {
    pub id: String,
    pub link: Link,
    pub context: WarningContext,
    pub code: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct WarningContext {
    pub details: Vec<String>,
    pub title: String,
    pub link_text: String,
}

#[derive(Deserialize, Debug)]
pub struct CancellationReason {
    pub message: String,
    pub code: String,
    pub error_code: String,
    pub error_cta: String,
}

#[derive(Deserialize, Debug)]
pub struct TaxDetail {
    pub name: String,
    pub amount: Balance,
}

#[derive(Deserialize, Debug)]
pub struct TradeIncentiveInfo {
    pub applied_incentive: bool,
    pub user_incentive_id: String,
    pub code_val: String,
    pub ends_at: String,
    pub fee_without_incentive: Balance,
    pub redeemed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeIncentiveMetadata {
    pub user_incentive_id: Option<String>,
    pub code_val: Option<String>,
}

/// Response from the convert API endpoint.
#[derive(Deserialize, Debug)]
pub(crate) struct TradeWrapper {
    pub(crate) trade: Trade,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ConvertQuoteQuery {
    /// The currency of the account to convert from, e.g. USD
    pub(crate) from_account: String,
    /// The currency of the account to convert to, e.g. USDC
    pub(crate) to_account: String,
    /// The amount to convert in the currency of the from_account
    pub(crate) amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) trade_incentive_metadata: Option<TradeIncentiveMetadata>,
}

/// Represents parameters to obtain a currency conversion.
#[derive(Serialize, Default, Debug)]
pub(crate) struct ConvertQuery {
    /// Originating account.
    pub(crate) from_account: String,
    /// Sending account.
    pub(crate) to_account: String,
}

impl Query for ConvertQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("from_account", &self.from_account)
            .push("to_account", &self.to_account)
            .build()
    }
}
