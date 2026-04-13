use super::Payment;
use super::Status;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};

/// Convenience methods
impl Payment {
    /// Is the payment url expired?
    /// INFO:
    /// For safety reasons,
    /// This method expires the payments a bit earlier than the nowpayment API
    /// (which is > 7 days).
    pub fn is_expired(&self) -> bool {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let diff = now - self.created_at;
        diff.num_days() > 4
    }
    pub fn is_used(&self) -> bool {
        vec![Status::Confirming, Status::Confirmed, Status::Sending].contains(&self.status)
    }
    pub fn is_finished(&self) -> bool {
        vec![
            Status::Finished,
            Status::PartiallyPaid,
            Status::Failed,
            Status::Refunded,
        ]
        .contains(&self.status)
    }
    // The payment status is unknown.
    pub fn is_unknown(&self) -> bool {
        vec![Status::Unknown].contains(&self.status)
    }

    pub async fn update(&mut self) -> Result<Self> {
        #[cfg(debug_assertions)]
        {
            self.status = Status::Sending;
        }
        #[cfg(not(debug_assertions))]
        {
            let client = EnvConfig::client();
            let updated_payment = client.payment().state().payment_id(self.id).get().await?;
            *self = updated_payment;
        }

        Ok(self.to_owned())
    }
}
