pub mod client;
pub use client::*;

pub mod jwt;
pub mod response;
pub use response::{Currency, Payment, Status};

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::client::Client;
    use crate::response::{status::ApiStatus, Currency, Payment, Status};

    use anyhow::Result;

    fn client() -> Client {
        Client::from_env().build()
    }

    fn sandbox_client() -> Client {
        Client::from_env().sandbox(true).build()
    }

    #[test]
    fn verify_client() {
        client();
    }

    #[test]
    fn verify_sandbox_client() {
        sandbox_client();
    }

    #[tokio::test]
    async fn get_status() -> Result<()> {
        let client = client();
        let status = client.status().await?;

        assert_eq!(status, ApiStatus::Running);
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().all().get().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_full_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().all_with_details().get().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_checked_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().allowed().get().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_min_payment_amount() -> Result<()> {
        let client = client();
        // panics if not error
        client
            .currencies()
            .min_amount()
            .from(&Currency::ETH)
            .to(&Currency::BTC)
            // Optional: default to USD.
            .fiat_equivalent(&Currency::USD)
            .get()
            .await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_estimate_price() -> Result<()> {
        let client = client();
        // panics if not error
        client
            .currencies()
            .price()
            .amount(2000.0)
            .from(&Currency::BTC)
            .to(&Currency::ETH)
            .get()
            .await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn failed_authentication() -> Result<()> {
        let mut client = client();

        // This step can be ignored when email and password are set directly from env via Client::from_env().build();
        let email = "test@test.org";
        let password = "my_password";
        client
            .auth()
            .credentials()
            .email(email)
            .password(password)
            .set();

        // Request a JWT against the remote API.
        assert!(client.auth().set().await.is_err());
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn authentication() -> Result<()> {
        let mut client = client();

        // Request a JWT against the remote API.
        client.auth().set().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn mock_create_payment() -> Result<()> {
        let client = client();
        client
            .mock()
            .payment()
            .create()
            .amount(100.0)
            .price_currency(&Currency::USD)
            .pay_currency(&Currency::XMR)
            .order_id("my_order_0")
            .order_description("my test order")
            .post()?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn create_payment() -> Result<()> {
        let client = client();
        let payment: Payment = client
            .payment()
            .create()
            .amount(100.0)
            .price_currency(&Currency::USD)
            .pay_currency(&Currency::XMR)
            .order_id("my_test_order_0")
            .order_description("nowpayments_rs::test::my_test_order")
            .ipn_callback_url("https://test.rs.nowpayments.io/")
            .post()
            .await?;
        // println!("{:#?}", payment);
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn get_payment() -> Result<()> {
        let mut client = client();
        client.auth().set().await?;
        client.payment().state().payment_id(1).get().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // TODO
    // WARNING: Method does not work on sandbox.
    async fn get_many_payments() -> Result<()> {
        Ok(())
    }
}
