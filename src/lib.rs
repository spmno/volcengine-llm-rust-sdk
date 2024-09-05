mod api;

use derive_builder::Builder;
use reqwest::{Client, RequestBuilder, Response};
use std::time::Duration;
use anyhow::{anyhow, Result};
use api::*;
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use bytes::Bytes;

const TIMEOUT: u64 = 60;

#[derive(Debug, Clone, Builder)]
pub struct LlmSdk {
    #[builder(setter(into), default = r#""/api/v3/chat/completions".into()"#)]
    pub(crate) base_url: String,
    pub(crate) key: String,
    pub(crate) client: Client
}

pub trait MessageEvent {
    fn on_message(&self, chunk: ChatCompletionChunkResponse);
    fn on_end(&self);
}

impl LlmSdk {
    pub fn new(key: String) -> Self {
        Self {
            key,
            client: Client::new(),
            base_url: "http://ark.cn-beijing.volces.com/api/v3".to_string(),
        }
    }

    pub async fn chat_completion(&self, req: &ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self.client.post(url)
            .json(req)
            .bearer_auth(&self.key)
            .timeout(Duration::from_secs(TIMEOUT));
        let res = request_build.send_and_log().await?;
        info!("chat completion response: {:?}", res);
        Ok(res.json::<ChatCompletionResponse>().await?)
    }

    pub async fn chat_completion_stream(&self, req: &ChatCompletionRequest,  event: &impl MessageEvent) -> Result<()>  {

        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self.client.post(url)
            .json(req)
            .bearer_auth(&self.key)
            .timeout(Duration::from_secs(TIMEOUT));
        let mut res = request_build.send().await?;
        info!("chat completion stream response: {:?}", res);
        while let Some(chunk) = res.chunk().await? {
            let chunk = serde_json::from_slice(&chunk)?;
            event.on_message(chunk);

        }
        event.on_end();
        Ok(())
    }

    pub async fn vision_lite(&self, req: &VisionLiteRequest) -> Result<VisionLiteResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self.client.post(url)
            .json(req)
            .bearer_auth(&self.key)
            .timeout(Duration::from_secs(TIMEOUT));
        let res = request_build.send_and_log().await?;
        info!("vision response: {:?}", res);
        Ok(res.json::<VisionLiteResponse>().await?)
    }

    //fn prepare_request(&self, req: &impl IntoRequest) -> &RequestBuilder {
    //    let req = req.into_request(&self.base_url, &self.client);
    //    let req = if self.key.is_empty() {
    //        req
    //    } else {
    //        req.bearer_auth(&self.key)
    //    };
    //    &req.timeout(Duration::from_secs(TIMEOUT))
    //}
}

trait SendAndLog {
    async fn send_and_log(self) -> Result<Response>;
}

impl SendAndLog for RequestBuilder {
    async fn send_and_log(self) -> Result<Response> {
        let res = self.send().await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            info!("status: {}", status);
            let text = res.text().await?;
            error!("API failed: {}", text);
            return Err(anyhow!("API failed: {}", text));
        }
        Ok(res)
    }
}


#[cfg(test)]
#[ctor::ctor]
fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();
}

#[cfg(test)]
lazy_static::lazy_static! {
    static ref SDK: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
}
