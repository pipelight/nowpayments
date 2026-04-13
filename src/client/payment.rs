use super::Client;
use crate::response::{
    payments::{EstimatedPaymentAmount, RawPayment, RawPayments},
    Currency, Payment, Status,
};
use chrono::{NaiveDateTime, Utc};
use convert_case::{Case, Casing};
use std::fmt;

use bon::bon;
use rust_decimal::{
    prelude::{FromPrimitive, FromStr, ToPrimitive},
    Decimal,
};
use std::collections::HashMap;

use anyhow::{bail, Result};

#[derive(Debug, serde::Deserialize)]
struct ApiError {
    status: bool,
    #[serde(alias = "statusCode", alias = "status_code")]
    status_code: Option<u16>,
    code: Option<String>,
    message: String,
}

pub trait DefaultPaymentMethods {
    fn create() -> Payment;
    fn state() -> Payment;
    // fn all() -> Vec<Payment>;
    // fn one() -> Payment;
}

/// Just a convenience pattern so that,
/// related methods are tidy under a common namespace.
///
/// example:
///
/// ```rs
/// client().payment().create();
/// client().payment().status();
/// ```
///
pub struct PaymentMethods<'a> {
    client: &'a Client,
}
impl Client {
    pub fn payment(&self) -> PaymentMethods<'_> {
        PaymentMethods { client: &self }
    }
}
#[bon]
impl PaymentMethods<'_> {
    #[builder(finish_fn = post)]
    #[tracing::instrument(skip_all)]
    /// Create a payment.
    pub async fn create(
        &self,
        amount: f64,
        price_currency: &Currency,
        pay_currency: &Currency,
        ipn_callback_url: &str,
        order_id: Option<&str>,
        order_description: Option<&str>,
    ) -> Result<Payment> {
        let mut body = HashMap::from([
            (
                "price_amount",
                Decimal::from_f64(amount).unwrap().to_string(),
            ),
            ("price_currency", price_currency.to_string()),
            ("pay_currency", pay_currency.to_string()),
            ("ipn_callback_url", ipn_callback_url.to_string()),
        ]);
        if let Some(order_id) = order_id {
            body.insert("order_id", order_id.to_string());
        }
        if let Some(order_description) = order_description {
            body.insert("order_description", order_description.to_string());
        }
        let client = self.client;
        let res: String = client.post("payment", body).await?;
        
        if let Ok(err) = serde_json::from_str::<ApiError>(&res) {
            if !err.status {
                let code = err.code.unwrap_or_else(|| "UNKNOWN".to_string());
                let msg = err.message;

                return Err(anyhow::anyhow!(
                    "NowPayments API error: [{}] {} (statusCode: {:?})",
                    code,
                    msg,
                    err.status_code
                ));
            }
        }

        let payment: RawPayment = serde_json::from_str(res.as_str())?;
        let payment: Payment = payment.into();
        Ok(payment)
    }

    #[builder(finish_fn = get)]
    #[tracing::instrument(skip_all)]
    /// Return an existing payment state.
    pub async fn state(&self, payment_id: u64) -> Result<Payment> {
        let client = self.client;
        if client.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!("payment/{}", payment_id);
        let res: String = self.client.get(&path).await?;
        let payment: RawPayment = serde_json::from_str(res.as_str())?;
        let mut payment: Payment = payment.into();

        // When payment is over,
        // Set paid amount to USD and persist.
        #[cfg(debug_assertions)]
        if payment.is_finished() {
            if let Some(actually_paid) = payment.actually_paid {
                let res: EstimatedPaymentAmount = client
                    .currencies()
                    .price()
                    .amount(actually_paid.to_f64().unwrap())
                    .from(&payment.pay_currency)
                    .to(&payment.price_currency)
                    .get()
                    .await?;
                payment.actually_paid_price = Some(Decimal::from_str(&res.estimated_amount)?);
            }
        }

        Ok(payment)
    }
    #[builder(finish_fn = get)]
    #[tracing::instrument(skip_all)]
    /// Return an existing payment state.
    pub async fn all(
        &self,
        limit: u64, // 1 to 500
        page: u64,  // page count from 0 to n
        sort_by: SortingField,
        order_by: OrderDirection,
        date_from: NaiveDateTime,
        date_to: NaiveDateTime,
    ) -> Result<Vec<Payment>> {
        let client = self.client;
        if client.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let from = date_from.format("%Y-%m-%d");
        let to = date_to.format("%Y-%m-%d");
        let path =
            format!("payment/?limit={limit}&page={page}&sortBy={sort_by}&orderBy={order_by}&dateFrom={from}&date_to={to}",);

        let res: String = self.client.get(&path).await?;
        let payments: RawPayments = serde_json::from_str(res.as_str())?;
        let payment: Vec<Payment> = payments.into();
        Ok(payment)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum SortingField {
    #[default]
    CreatedAt,
    PaymentId,
    PaymentStatus,
    PayAddress,
    PriceAmount,
    PriceCurrency,
    PayAmount,
    PayCurrency,
    ActuallyPaid,
    OrderId,
    OrderDescription,
    PurchaseId,
    OutcomeAmount,
    OutcomeCurrency,
}
impl fmt::Display for SortingField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_case(Case::Snake))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    #[default]
    Desc,
    Asc,
}
impl fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_case(Case::Snake))
    }
}
