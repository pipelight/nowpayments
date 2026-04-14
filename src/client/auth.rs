use super::{Client, Currency};
use crate::jwt::JWTJson;

use bon::bon;
use std::collections::HashMap;

use anyhow::{bail, Result};

/// Just a convenience pattern so that,
/// related methods are tidy under a common namespace.
///
/// example:
///
/// ```rs
/// client().auth();
/// ```
///
pub struct AuthMethods<'a> {
    client: &'a mut Client,
}
impl Client {
    pub fn auth(&mut self) -> AuthMethods<'_> {
        AuthMethods { client: self }
    }
}
#[bon]
impl AuthMethods<'_> {
    #[builder(finish_fn = get)]
    #[tracing::instrument(skip_all)]
    pub async fn jwt(&self) {
        dbg!(&self.client.jwt);
    }
    #[builder(
        finish_fn = set,
        on(String,into),
        on(Option<String>,into)
    )]
    pub fn credentials(&mut self, email: Option<String>, password: Option<String>) {
        self.client.email = email;
        self.client.password = password;
    }
    // Get a JWT from the API.
    // Needed for calls that need some priviledges
    //
    // WARNING: Use only alphanumeric passwords.
    // There is an issue with reqwest and serde sanitizing json,
    // thus passwords with special chars won't work with the API.
    #[tracing::instrument(skip_all)]
    pub async fn set(&mut self) -> Result<()> {
        let client = &mut self.client;
        if client.email.is_none() || client.password.is_none() {
            bail!("You did not set an email or a password.");
        }

        // Here the order matter for the later json object generation.
        // Maybe an indexmap would be better if things get more .
        let mut json = HashMap::new();
        json.insert("email", client.email.clone().unwrap());
        json.insert("password", client.password.clone().unwrap());

        let data = client.post("auth", json).await?;
        let jwt: JWTJson = serde_json::from_str(&data)?;
        tracing::trace!("Issued new jwt: {:#?}", jwt);

        client.jwt.set(jwt.token);
        Ok(())
    }
}
