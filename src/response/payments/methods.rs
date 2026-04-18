use super::Payment;
use super::Status;
use crate::Client;

use anyhow::Result;
use bon::{bon, builder};
use chrono::{NaiveDateTime, Utc};

/// Convenience methods
impl Payment {
    /// Is the payment url expired?
    /// INFO:
    /// For safety reasons,
    /// This method expires the payments a bit earlier than the nowpayment API:
    /// - Production API: > 7 days.
    /// - Sandbox API: > 24 hours.
    pub fn is_expired(&self) -> bool {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let diff = now - self.created_at;
        self.status == Status::Expired || diff.num_days() > 4
    }
    /// The payment is being used.
    /// Some funds have been or are being sent.
    pub fn is_used(&self) -> bool {
        vec![Status::Confirming, Status::Confirmed, Status::Sending].contains(&self.status)
    }
    /// The payment is finished.
    /// Some funds have been received, or the payment has failed.
    pub fn is_finished(&self) -> bool {
        vec![
            Status::Finished,
            Status::PartiallyPaid,
            Status::Failed,
            Status::Refunded,
        ]
        .contains(&self.status)
    }
    /// The payment status is unknown.
    pub fn is_unknown(&self) -> bool {
        vec![Status::Unknown].contains(&self.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Currency;
    use anyhow::Result;
    use chrono::Days;
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_is_methods() -> Result<()> {
        let payment = Payment {
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

        assert_eq!(payment.is_expired(), true);
        assert_eq!(payment.is_used(), false);
        assert_eq!(payment.is_finished(), false);

        let now: NaiveDateTime = Utc::now().naive_utc();
        let yesterday = now.checked_sub_days(Days::new(1)).unwrap();
        let payment = Payment {
            id: 0,
            status: Status::Sending,
            address: "my_fake_address".to_string(),

            price_amount: Decimal::from_f64(10.0).unwrap(),
            price_currency: Currency::USD,
            pay_amount: Decimal::from_f64(0.01).unwrap(),
            pay_currency: Currency::XMR,

            actually_paid: Some(Decimal::from_f64(0.005).unwrap()),
            actually_paid_price: None,

            order_id: "test_id".to_string(),
            order_description: "my test".to_string(),

            created_at: yesterday,
            updated_at: yesterday,
        };

        assert_eq!(payment.is_expired(), false);
        assert_eq!(payment.is_used(), true);
        assert_eq!(payment.is_finished(), false);

        Ok(())
    }
}
