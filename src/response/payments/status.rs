use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// See nowpayment "Get payment status" method "https://api.nowpayments.io/v1/payment/:payment_id"
///
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Status {
    #[default]
    /// Couldn't retrieve a status from remote API.
    Unknown,
    /// A testing status for this library.
    Dummy,
    /// The initial status of each payment.
    /// Waiting for the customer to send the payment.
    Waiting,
    /// The transaction is being processed on the blockchain.
    /// Appears when NOWPayments detect the funds from the user on the blockchain.
    Confirming,
    /// The process is confirmed by the blockchain.
    /// Customer’s funds have accumulated enough confirmations.
    Confirmed,
    /// The funds are being sent to your personal wallet.
    /// NOWpayments is in the process of sending the funds to you.
    Sending,
    /// It shows that the customer sent the less than the actual price.
    /// Appears when the funds have arrived in your wallet.
    PartiallyPaid,
    /// The funds have reached your personal address and the payment is finished.
    Finished,
    /// The payment wasn't completed due to the error of some kind.
    Failed,
    /// The funds were refunded back to the user.
    Refunded,
    /// The user didn't send the funds to the specified address,
    /// in the 24 hour time window.
    Expired,
}
impl FromStr for Status {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let res = match value {
            "waiting" => Self::Waiting,
            "confirming" => Self::Confirming,
            "confirmed" => Self::Confirmed,
            "sending" => Self::Sending,
            "partially_paid" => Self::PartiallyPaid,
            "finished" => Self::Finished,
            "failed" => Self::Failed,
            "refunded" => Self::Refunded,
            "expired" => Self::Expired,
            "dummy" => Self::Dummy,
            _ => Self::Unknown,
        };
        Ok(res)
    }
}
