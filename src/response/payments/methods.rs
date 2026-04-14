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
    /// This method expires the payments a bit earlier than the nowpayment API
    /// (which is > 7 days).
    pub fn is_expired(&self) -> bool {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let diff = now - self.created_at;
        diff.num_days() > 4
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

#[bon]
impl Payment {
    #[builder(finish_fn = exec)]
    pub async fn update(&mut self, mock: Option<bool>) -> Result<Self> {
        match mock {
            Some(true) => {
                self.status = Status::Sending;
            }
            _ => {
                let client = Client::from_env().build();
                let updated_payment = client.payment().state().payment_id(self.id).get().await?;
                *self = updated_payment;
            }
        };
        Ok(self.to_owned())
    }
}
