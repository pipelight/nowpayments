mod auth;
mod currencies;
mod http;
mod mock_payment;
mod payment;
mod payout;

use bon::{bon, builder};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// Env vars
use dotenvy;
use std::env::var;
use std::path::Path;

use crate::response::{
    conversion::SingleConversion,
    payments::Status,
    status::{ApiStatus, RawApiStatus},
};

use crate::response::payments::EstimatedPaymentAmount;
use crate::response::payments::MinPaymentAmount;

use crate::response::{currencies::Currency, payments::Payment};

use crate::{
    jwt::{JWTJson, JWT},
    response::conversion::AllConversions,
};
use anyhow::{bail, Result};
use reqwest::header;

static BASE_URL: &str = "https://api.nowpayments.io/v1/";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.2.3");

pub struct Client {
    base_url: &'static str,
    email: Option<String>,
    password: Option<String>,
    jwt: JWT,
    client: reqwest::Client,
}

#[bon]
impl Client {
    /// Load the .env file from project root or from the path parameter.
    #[builder(
        on(String,into),
        on(Option<String>,into)
    )]
    pub fn from_env(path: Option<String>, sandbox: Option<bool>) -> Self {
        match path {
            None => {
                dotenvy::dotenv().unwrap();
            }
            Some(v) => {
                dotenvy::from_path(Path::new(&v)).unwrap();
            }
        };
        Client::builder()
            .api_key(var("NOWPAYMENTS_API_KEY").unwrap())
            .maybe_sandbox(sandbox)
            .maybe_email(var("NOWPAYMENTS_EMAIL").ok())
            .maybe_password(var("NOWPAYMENTS_PASSWORD").ok())
            .build()
    }

    #[builder(
        on(String,into),
        on(Option<String>,into)
    )]
    pub fn new(
        api_key: String,
        email: Option<String>,
        password: Option<String>,
        sandbox: Option<bool>,
    ) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&api_key).unwrap(),
        );
        let base_url = match sandbox {
            Some(true) => BASE_SANDBOX_URL,
            _ => BASE_URL,
        };
        Self {
            base_url,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
            email,
            password,
            jwt: JWT::new(),
        }
    }
}

impl Client {
    pub async fn status(&self) -> Result<ApiStatus> {
        let res = self.get("status").await?;
        let status: RawApiStatus = serde_json::from_str(res.as_str())?;
        let status: ApiStatus = status.into();
        Ok(status)
    }
}

impl Client {
    // TODO
    pub async fn get_balance(&self) -> Result<Status> {
        let req = self.get("balance").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_list_of_payments(
        &self,
        limit: impl Display,
        page: impl Display,
        sort_by: impl Display,
        order_by: impl Display,
        date_from: impl Display,
        date_to: impl Display,
    ) -> Result<Payment> {
        if self.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!(
            "payment/?limit={}&page={}&sortBy={}&orderBy={}&dateFrom={}&dateTo={}",
            limit, page, sort_by, order_by, date_from, date_to
        );
        let req = self.get(&path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_status(
        &self,
        conversion_id: impl Display,
    ) -> Result<SingleConversion> {
        let path = format!("conversion/{}", conversion_id);
        let req = self.get(&path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_list(&self) -> Result<AllConversions> {
        let path = "conversion".to_string();
        let req = self.get(&path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
}
