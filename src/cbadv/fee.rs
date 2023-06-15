use crate::cbadv::utils::Signer;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FeeTier {
    pub pricing_tier: String,
    pub usd_from: String,
    pub usd_to: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MarginRate {
    pub value: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
    pub value: String,
    pub r#type: String,
}

/// Represents the transaction summary for fees received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionSummary {
    pub total_volume: f64,
    pub total_fees: f64,
    pub fee_tier: FeeTier,
    pub margin_rate: Option<MarginRate>,
    pub goods_and_services_tax: Option<Tax>,
    pub advanced_trade_only_volume: f64,
    pub advanced_trade_only_fees: f64,
    pub coinbase_pro_volume: f64,
    pub coinbase_pro_fees: f64,
}

/// Represents parameters that are optional for transaction summary API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct TransactionSummaryParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub user_native_currency: Option<String>,
    pub product_type: Option<String>,
}

impl TransactionSummaryParams {
    pub fn to_params(&self) -> String {
        let mut has_prior: bool = false;

        let start_date = match &self.start_date {
            Some(v) => {
                has_prior = true;
                format!("start_date={}", v)
            }
            None => "".to_string(),
        };

        let end_date = match &self.end_date {
            Some(v) => {
                let mut sep = "";
                if has_prior {
                    sep = "&"
                }
                has_prior = true;

                format!("{}end_date={}", sep, v)
            }
            None => "".to_string(),
        };

        let native = match &self.user_native_currency {
            Some(v) => {
                let mut sep = "";
                if has_prior {
                    sep = "&"
                }
                has_prior = true;

                format!("{}user_native_currency={}", sep, v)
            }
            None => "".to_string(),
        };

        let product = match &self.product_type {
            Some(v) => {
                let mut sep = "";
                if has_prior {
                    sep = "&"
                }

                format!("{}product_type={}", sep, v)
            }
            None => "".to_string(),
        };

        // format!("limit={}&cursor={}", self.limit, self.cursor)
        format!("{}{}{}{}", start_date, end_date, native, product)
    }
}

/// Provides access to the Fee API for the service.
pub struct FeeAPI {
    signer: Signer,
}

impl FeeAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/transaction_summary";

    /// Creates a new instance of the Fee API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains fee transaction summary from the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/transaction_summary
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gettransactionsummary
    pub async fn get(&self, params: TransactionSummaryParams) -> Result<TransactionSummary> {
        let resource = Self::RESOURCE.to_string();
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<TransactionSummary>().await {
                Ok(resp) => Ok(resp),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get fee transaction summary: {}", error);
                Err(error)
            }
        }
    }
}
