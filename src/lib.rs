mod api;

use anyhow::{anyhow, Result};
use api::*;
use bytes::Bytes;
use derive_builder::Builder;
use reqwest::{Client, RequestBuilder, Response};
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const TIMEOUT: u64 = 60;
// 换行符号
const LINE_FEED: u8 = 10;
// 多搜索的位置，优化搜索使用
const SEARCH_TAIL: usize = 10;
// 换行符号的个数，在返回json后面有两个换行符
const LINE_FEED_COUNT: usize = 2;
// 大括号的ascii码，查找有效json数据使用
const LEFT_SIGN: u8 = 123;

#[derive(Debug, Clone, Builder)]
pub struct LlmSdk {
    #[builder(setter(into), default = r#""/api/v3/chat/completions".into()"#)]
    pub(crate) base_url: String,
    pub(crate) key: String,
    pub(crate) client: Client,
}

pub trait MessageEvent {
    fn on_message(&self, chat_completion: &ChatCompletionChunkResponse);
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

    pub async fn chat_completion(
        &self,
        req: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self
            .client
            .post(url)
            .json(req)
            .bearer_auth(&self.key)
            .timeout(Duration::from_secs(TIMEOUT));
        let res = request_build.send_and_log().await?;
        info!("chat completion response: {:?}", res);
        Ok(res.json::<ChatCompletionResponse>().await?)
    }

    pub async fn chat_completion_stream(
        &self,
        req: &ChatCompletionRequest,
        event: &impl MessageEvent,
    ) -> Result<()> {
        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self
            .client
            .post(url)
            .json(req)
            .bearer_auth(&self.key)
            .timeout(Duration::from_secs(TIMEOUT));
        let mut res = request_build.send().await?;
        info!("chat completion stream response: {:?}", res);
        while let Some(mut chunk) = res.chunk().await? {
            info!("chunk:{:?}", chunk);
            Self::dispose_chunk(chunk, event);
 
        }
        Ok(())
    }

    fn dispose_chunk(mut chunk: Bytes, event: &impl MessageEvent)  {
        // 多帧的处理
        let chunk_len = chunk.len();
        // 让搜索少一点吧
        let search_len = chunk_len / 2 + SEARCH_TAIL;
        let mut line_count = 0;
        let mut last_pos = 0;
        for i in 0..search_len {
            // 找出换行，查看后面是否还有数据
            if chunk[i] == LINE_FEED {
                if i < chunk_len - LINE_FEED_COUNT {
                    info!("multi frame: {},{}", i, chunk[i + LINE_FEED_COUNT]);
                    last_pos = i;
                    if (last_pos + 1) == i {
                        line_count = line_count + 1;
                    }
                    let previous_json_bytes = chunk.split_to(i+LINE_FEED_COUNT);
                    let mut pos = 0;
                    for i in 0..previous_json_bytes.len() {
                        if previous_json_bytes.get(i).unwrap().eq(&LEFT_SIGN) {
                            pos = i;
                            break;
                        }
                    }
                    let chat_completion: ChatCompletionChunkResponse =
                        serde_json::from_slice(&previous_json_bytes[pos..]).unwrap();
                    event.on_message(&chat_completion);
                    break;
                }
            }
        }
        if last_pos > 0 {
            Self::dispose_chunk(chunk, event);
          } else {
            let mut pos = 0;
            for i in 0..chunk.len() {
                if chunk.get(i).unwrap().eq(&LEFT_SIGN) {
                    pos = i;
                    break;
                }
            }
            if pos > 0 {
                if (chunk[chunk_len-4] == 69) && (chunk[chunk_len-5] == 78) && (chunk[chunk_len-6] == 79) {
                    let previous_json_bytes = chunk.split_to(chunk_len - "data: [DONE]\n\n".len());
                    let chat_completion: ChatCompletionChunkResponse = serde_json::from_slice(&previous_json_bytes[pos..]).unwrap();
                    event.on_message(&chat_completion);
                    event.on_end();
                } else {
                    let chat_completion: ChatCompletionChunkResponse = serde_json::from_slice(&chunk[pos..]).unwrap();
                    event.on_message(&chat_completion);
                }

            } 
        }
        
    }

    pub async fn vision_lite(&self, req: &VisionLiteRequest) -> Result<VisionLiteResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        info!("url:{}", url);
        let request_build = self
            .client
            .post(url)
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
    tracing_subscriber::registry().with(fmt::layer()).init();
}

#[cfg(test)]
lazy_static::lazy_static! {
    static ref SDK: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
}
