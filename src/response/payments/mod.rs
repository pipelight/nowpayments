mod methods;
mod status;
pub use status::Status;

use crate::response::Currency;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Payment {
    pub id: u64,
    pub status: Status,
    /// Receiving address.
    pub address: String,

    /// The article price
    pub price_amount: Decimal,
    /// The article currency
    pub price_currency: Currency,
    /// The transaction amount
    pub pay_amount: Decimal,
    /// The transaction currency
    pub pay_currency: Currency,

    /// Paid amount in currency from pay_amount(any crypto)
    pub actually_paid: Option<Decimal>,

    /// INFO:
    /// This field does not exists in the NOWPayments API.
    /// This is a convenience field added by this library and transparently populated by the "client.payment().state().payment_id(payment_id).get()" method.
    ///
    /// Paid amount in currency from price_amount(usd)
    pub actually_paid_price: Option<Decimal>,

    /// Extra informations.
    /// order_id should be: <account_uuid>-<currency>
    pub order_id: String,
    pub order_description: String,

    // Date
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<RawPayment> for Payment {
    /// Convert NowPayments Json API response to a convenient struct with methods.
    fn from(e: RawPayment) -> Self {
        Self {
            id: e.payment_id.parse().unwrap(),
            status: Status::from_str(&e.payment_status).unwrap(),
            address: e.pay_address,

            price_amount: e.price_amount,
            price_currency: Currency::from_str(&e.price_currency).unwrap(),
            pay_amount: e.pay_amount,
            pay_currency: Currency::from_str(&e.pay_currency).unwrap(),

            actually_paid: e.actually_paid,
            actually_paid_price: None,

            order_id: e.order_id,
            order_description: e.order_description.unwrap_or_default(),

            created_at: NaiveDateTime::parse_from_str(&e.created_at, "%Y-%m-%dT%H:%M:%S%.3fZ")
                .unwrap(),
            updated_at: NaiveDateTime::parse_from_str(&e.updated_at, "%Y-%m-%dT%H:%M:%S%.3fZ")
                .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinPaymentAmount {
    pub currency_from: String,
    pub currency_to: String,
    pub min_amount: Decimal,
    pub fiat_equivalent: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedPaymentAmount {
    pub currency_from: String,
    pub amount_from: Decimal,
    pub currency_to: String,
    pub estimated_amount: String,
}

/// Response from the /create-payment endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPayment {
    pub payment_id: String, // must contain numbers only (u64)
    pub payment_status: String,
    pub pay_address: String,

    pub price_amount: Decimal,
    pub price_currency: String,
    pub pay_amount: Decimal,
    pub pay_currency: String,

    pub actually_paid: Option<Decimal>,

    pub order_id: String,
    pub order_description: Option<String>,
    pub purchase_id: String,
    // Dates
    pub created_at: String,
    pub updated_at: String,
}

/// Response from the /create-payment endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPayments {
    data: Vec<RawPayment>,
}
impl From<RawPayments> for Vec<Payment> {
    fn from(value: RawPayments) -> Self {
        let res: Vec<Payment> = value.data.iter().map(|e| e.to_owned().into()).collect();
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use rust_decimal::prelude::FromPrimitive;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn convert() -> Result<()> {
        let api_res = RawPayment {
            payment_id: "0".to_string(),
            payment_status: "waiting".to_string(),
            pay_address: "my_fake_address".to_string(),

            price_amount: Decimal::from_f64(10.0).unwrap(),
            price_currency: "usd".to_string(),
            pay_amount: Decimal::from_f64(0.01).unwrap(),
            pay_currency: "xmr".to_string(),

            actually_paid: Some(Decimal::from_f64(0.005).unwrap()),

            order_id: "test_id".to_string(),
            order_description: Some("my test".to_string()),

            purchase_id: "".to_string(),

            created_at: "2026-01-10T17:56:15.327Z".to_string(),
            updated_at: "2026-01-10T17:56:15.327Z".to_string(),
        };

        let converted_res = Payment {
            id: 0,
            status: Status::Waiting,
            address: "my_fake_address".to_string(),

            price_amount: Decimal::from_f64(10.0).unwrap(),
            price_currency: Currency::USD,
            pay_amount: Decimal::from_f64(0.01).unwrap(),
            pay_currency: Currency::XMR,

            actually_paid: Some(Decimal::from_f64(0.005).unwrap()),
            actually_paid_price: None,

            order_id: "test_id".to_string(),
            order_description: "my test".to_string(),

            created_at: NaiveDateTime::parse_from_str(
                "2026-01-10T17:56:15.327Z",
                "%Y-%m-%dT%H:%M:%S%.3fZ",
            )
            .unwrap(),
            updated_at: NaiveDateTime::parse_from_str(
                "2026-01-10T17:56:15.327Z",
                "%Y-%m-%dT%H:%M:%S%.3fZ",
            )
            .unwrap(),
        };

        assert_eq!(converted_res, Payment::from(api_res));

        Ok(())
    }
}
