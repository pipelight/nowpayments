use super::{Client, Currency};
use crate::response::currencies::{Currencies, FullCurrencies, SelectedCurrencies};
use crate::response::payments::{EstimatedPaymentAmount, MinPaymentAmount};

use bon::bon;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use anyhow::{bail, Result};

pub struct CurrenciesMethods<'a> {
    client: &'a Client,
}

impl Client {
    pub fn currencies(&self) -> CurrenciesMethods<'_> {
        CurrenciesMethods { client: &self }
    }
}
#[bon]
impl CurrenciesMethods<'_> {
    #[tracing::instrument(skip_all)]
    /// Create a payment.

    #[builder(finish_fn = get)]
    pub async fn all(&self) -> Result<Currencies> {
        let client = self.client;
        let req = client.get("currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    #[builder(finish_fn = get)]
    pub async fn all_with_details(&self) -> Result<FullCurrencies> {
        let client = self.client;
        let req = client.get("full-currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    /// Get checked currencies.
    #[builder(finish_fn = get)]
    pub async fn allowed(&self) -> Result<SelectedCurrencies> {
        let client = self.client;
        let req = client.get("merchant/coins").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    /// Call to the /get_estimated_price API endpoint
    #[builder(
        finish_fn = get,
    )]
    pub async fn price(
        &self,
        amount: f64,
        from: &Currency,
        to: &Currency,
    ) -> Result<EstimatedPaymentAmount> {
        let path = format!(
            "estimate?amount={}&currency_from={}&currency_to={}",
            Decimal::from_f64(amount).unwrap(),
            from.to_string().to_lowercase(),
            to.to_string().to_lowercase()
        );
        let client = self.client;
        let req = client.get(&path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    // Get minimal payment amount.
    #[builder(finish_fn = get)]
    pub async fn min_amount(
        &self,
        from: &Currency,
        to: &Currency,
        fiat_equivalent: Option<&Currency>,
    ) -> Result<MinPaymentAmount> {
        let mut path = format!(
            "min-amount?currency_from={}&currency_to={}",
            from.to_string().to_lowercase(),
            to.to_string().to_lowercase()
        );
        let fiat_equivalent = match fiat_equivalent {
            Some(v) => v,
            None => &Currency::USD,
        };
        path.push_str(&format!(
            "&fiat_equivalent={}",
            fiat_equivalent.to_string().to_lowercase()
        ));
        let client = self.client;
        let res = client.get(&path).await?;

        Ok(serde_json::from_str(res.as_str())?)
    }
}
