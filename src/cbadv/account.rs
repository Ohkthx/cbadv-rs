use crate::cbadv::utils::{CBAdvError, Result, Signer};
use serde::{Deserialize, Serialize};

/// Represents a Balance for either Available or Held funds.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub value: String,
    pub currency: String,
}

/// Represents an Account received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub uuid: String,
    pub name: String,
    pub currency: String,
    pub available_balance: Balance,
    pub default: bool,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub r#type: String,
    pub ready: bool,
    pub hold: Balance,
}

/// Represents a list of accounts received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListAccounts {
    pub accounts: Vec<Account>,
    pub has_next: bool,
    pub cursor: String,
    pub size: i32,
}

/// Represents an account response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    pub account: Account,
}

/// Represents parameters that are optional for List Account API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListAccountsParams {
    /// Amount to obtain, default 49 maximum is 250.
    pub limit: Option<i32>,
    /// Returns accounts after the cursor provided.
    pub cursor: Option<String>,
}

impl ListAccountsParams {
    /// Converts the object into HTTP request parameters.
    pub fn to_params(&self) -> String {
        let mut params: String = "".to_string();

        params = match &self.limit {
            Some(v) => format!("{}&limit={}", params, v),
            _ => params,
        };

        params = match &self.cursor {
            Some(v) => format!("{}&cursor={}", params, v),
            _ => params,
        };

        match params.is_empty() {
            true => params,
            false => params[1..].to_string(),
        }
    }
}

/// Provides access to the Account API for the service.
pub struct AccountAPI {
    signer: Signer,
}

impl AccountAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/accounts";

    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains a single account based on the Account UUID (ex. "XXXX-YYYY-ZZZZ").
    ///
    /// # Arguments
    ///
    /// * `account_uuid` - A string the represents the account's UUID.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/accounts/{account_uuid}
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccount
    pub async fn get(&self, account_uuid: String) -> Result<Account> {
        let resource = format!("{}/{}", Self::RESOURCE.to_string(), account_uuid);
        match self.signer.get(resource, "".to_string()).await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(_) => Err(CBAdvError::BadParse("account object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains various accounts from the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/accounts
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccounts
    pub async fn get_all(&self, params: ListAccountsParams) -> Result<ListAccounts> {
        let resource = Self::RESOURCE.to_string();
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<ListAccounts>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("accounts vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
