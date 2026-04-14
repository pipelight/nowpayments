use super::Client;
use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;

// Internal http methods.
impl Client {
    #[tracing::instrument(skip_all)]
    pub async fn get(&self, endpoint: &str) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint.to_string());

        let req = self
            .client
            .get(endpoint)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .build()?;

        let response = self.client.execute(req).await?;
        // Print headers only (no body)
        tracing::debug!("{:#?}", response);

        let body_str = response.text().await?;
        let body_json: Value = serde_json::from_str(&body_str)?;
        // Print body
        tracing::debug!("{:#?}", body_json);
        tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);

        Ok(body_str)
    }

    #[tracing::instrument(skip_all)]
    pub async fn post(
        &self,
        endpoint: &str,
        data: HashMap<&'static str, String>,
    ) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint);

        let req = self
            .client
            .post(endpoint)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .json(&data)
            .build()?;

        // Print headers only (no body)
        // tracing::debug!("{:#?}", req);
        if let Some(body) = req.body() {
            let body_str = str::from_utf8(body.as_bytes().unwrap()).unwrap();
            let body_json: Value = serde_json::from_str(body_str)?;
            // Print body
            // tracing::debug!("{:#?}", body_json);
            // tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);
        }

        let response = self.client.execute(req).await?;
        // Print headers only (no body)
        tracing::debug!("{:#?}", response);

        let body_str = response.text().await?;
        let body_json: Value = serde_json::from_str(&body_str)?;
        // Print body
        tracing::debug!("{:#?}", body_json);
        tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);

        Ok(body_str)
    }
}
