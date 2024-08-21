mod api;

use derive_builder::Builder;
use reqwest::{Client, RequestBuilder, Response};
use std::time::Duration;
use anyhow::{anyhow, Result};
use api::*;

const TIMEOUT: u64 = 60;

#[derive(Debug, Clone, Builder)]
pub struct LlmSdk {
    #[builder(setter(into), default = r#""/api/v3/chat/completions".into()"#)]
    pub(crate) base_url: String,
    pub(crate) key: String,
    pub(crate) client: Client
}

pub trait IntoRequest {
    fn into_request(self, base_url: &str, client: Client) -> RequestBuilder;
}

impl LlmSdk {
    pub fn new(key: String) -> Self {
        Self {
            key,
            client: Client::new(),
            base_url: "/api/v3/chat/completions".to_string(),
        }
    }

    pub async fn chat_completion(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let req = self.prepare_request(req);
        let res = req.send_and_log().await?;
        Ok(res.json::<ChatCompletionResponse>().await?)
    }

    fn prepare_request(&self, req: impl IntoRequest) -> RequestBuilder {
        let req = req.into_request(&self.base_url, self.client.clone());
        let req = if self.key.is_empty() {
            req
        } else {
            req.bearer_auth(&self.key)
        };
        req.timeout(Duration::from_secs(TIMEOUT))
    }
}

trait SendAndLog {
    async fn send_and_log(self) -> Result<Response>;
}

impl SendAndLog for RequestBuilder {
    async fn send_and_log(self) -> Result<Response> {
        let res = self.send().await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            let text = res.text().await?;
            //error!("API failed: {}", text);
            return Err(anyhow!("API failed: {}", text));
        }
        Ok(res)
    }
}