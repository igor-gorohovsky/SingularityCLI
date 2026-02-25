use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    base_url: String,
    token: String,
    http: Client,
}

impl ApiClient {
    pub fn new(token: String) -> Self {
        Self {
            base_url: "https://api.singularity-app.com".to_string(),
            token,
            http: Client::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(token: String, base_url: String) -> Self {
        Self {
            base_url,
            token,
            http: Client::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str, query: &[(&str, String)]) -> Result<T> {
        let resp = self
            .http
            .get(self.url(path))
            .bearer_auth(&self.token)
            .query(query)
            .send()
            .context("request failed")?;

        Self::handle_response(resp)
    }

    pub fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let resp = self
            .http
            .post(self.url(path))
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .context("request failed")?;

        Self::handle_response(resp)
    }

    pub fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let resp = self
            .http
            .patch(self.url(path))
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .context("request failed")?;

        Self::handle_response(resp)
    }

    pub fn delete(&self, path: &str) -> Result<()> {
        let resp = self
            .http
            .delete(self.url(path))
            .bearer_auth(&self.token)
            .send()
            .context("request failed")?;

        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let body = resp.text().unwrap_or_default();
            bail!("API error ({}): {}", status, body)
        }
    }

    fn handle_response<T: DeserializeOwned>(resp: reqwest::blocking::Response) -> Result<T> {
        let status = resp.status();
        if status == StatusCode::UNAUTHORIZED {
            bail!("unauthorized — check your API token");
        }
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("API error ({}): {}", status, body);
        }
        resp.json::<T>().context("failed to parse response")
    }
}
