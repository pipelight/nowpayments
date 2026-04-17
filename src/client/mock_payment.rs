use super::Client;
use crate::response::{Currency, Payment, Status};
use bon::bon;
use chrono::{NaiveDateTime, Utc};
use rust_decimal::{prelude::FromPrimitive, Decimal};

use anyhow::Result;

pub struct MockMethods<'a> {
    client: &'a Client,
}
pub struct MockPaymentMethods<'a> {
    client: &'a Client,
}
impl Client {
    pub fn mock(&self) -> MockMethods<'_> {
        MockMethods { client: &self }
    }
}
impl MockMethods<'_> {
    pub fn payment(&self) -> MockPaymentMethods<'_> {
        MockPaymentMethods {
            client: self.client,
        }
    }
}
#[bon]
impl MockPaymentMethods<'_> {
    /// Create a payment locally.
    /// For testing purpose.
    #[builder(finish_fn = post)]
    #[tracing::instrument(skip_all)]
    pub fn create(
        &self,
        amount: f64,
        price_currency: &Currency,
        pay_currency: &Currency,
        order_id: Option<&str>,
        order_description: Option<&str>,

        status: Option<&Status>,
        actually_paid: Option<f64>,
    ) -> Result<Payment> {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let mut payment = Payment {
            id: 0,
            status: Status::Dummy,
            address: format!("<mock/my_{}_address>", pay_currency.network()),

            price_amount: Decimal::from_f64(amount).unwrap(),
            price_currency: price_currency.to_owned(),
            pay_amount: Decimal::from_f64(amount / 500.0).unwrap(),
            pay_currency: pay_currency.to_owned(),

            actually_paid: Some(Decimal::from_f64(amount / 1000.0).unwrap()),
            actually_paid_price: None,

            order_id: "".to_string(),
            order_description: "".to_string(),
            created_at: now,
            updated_at: now,
        };

        if let Some(order_id) = order_id {
            payment.order_id = order_id.to_string();
        }
        if let Some(order_description) = order_description {
            payment.order_description = order_description.to_string();
        }

        if let Some(status) = status {
            payment.status = status.to_owned();
        }
        if let Some(actually_paid) = actually_paid {
            payment.actually_paid = Some(Decimal::from_f64(actually_paid).unwrap());
        }

        Ok(payment)
    }
}
