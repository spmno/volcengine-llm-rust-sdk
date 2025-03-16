
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// https://www.volcengine.com/docs/82379/1298454#%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84

#[derive(Serialize, Clone, Debug, Builder)]
pub struct ChatCompletionRequest {
    /// 您创建的推理接入点 ID, ep-202406040*****-*****
    model: String,
    /// 由目前为止的对话组成的消息列表,当指定了 tools 参数以使用模型的 function call 能力时，
    /// 请确保 messages 列表内的消息满足如下要求：
    /// 如果 message 列表中前文出现了带有 n 个 tool_calls 的 Assistant Message，
    /// 则后文必须有连续 n 个分别和每个 tool_call_id 相对应的 Tool Message，来回应 tool_calls 的信息要求
    #[builder(setter(into))]
    messages: Vec<ChatCompletionMessage>,
    /// 响应内容是否流式返回
    /// false：模型生成完所有内容后一次性返回结果
    /// true：按 SSE 协议逐块返回模型生成内容，并以一条 data: [DONE] 消息结束
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    ///流式响应的选项。仅当 stream: true 时可以设置 stream_options 参数。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<StreamOptionsParam>,
    /// 模型可以生成的最大 token 数量。取值范围为 [0, 4096]。输入 token 和输出 token 的总长度还受模型的上下文长度限制。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    /// 模型遇到 stop 字段所指定的字符串时将停止继续生成，这个词语本身不会输出。最多支持 4 个字符串。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    /// 频率惩罚系数。如果值为正，会根据新 token 在文本中的出现频率对其进行惩罚，从而降低模型逐字重复的可能性。取值范围为 [-2.0, 2.0]。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    /// 采样温度。控制了生成文本时对每个候选词的概率分布进行平滑的程度。取值范围为 [0, 1]。当取值为 0 时模型仅考虑对数概率最大的一个 token。
    /// 较高的值（如 0.8）会使输出更加随机，而较低的值（如 0.2）会使输出更加集中确定。通常建议仅调整 temperature 或 top_p 其中之一，不建议两者都修改。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// 核采样概率阈值。模型会考虑概率质量在 top_p 内的 token 结果。取值范围为 [0, 1]。当取值为 0 时模型仅考虑对数概率最大的一个 token。
    /// 如 0.1 意味着只考虑概率质量最高的前 10% 的 token，取值越大生成的随机性越高，取值越低生成的确定性越高。通常建议仅调整 temperature 或 top_p 其中之一，不建议两者都修改。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// 是否返回输出 tokens 的对数概率。
    /// false：不返回对数概率信息
    /// true：返回消息内容中每个输出 token 的对数概率
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,
    /// 指定每个输出 token 位置最有可能返回的 token 数量，每个 token 都有关联的对数概率。
    /// 仅当 logprobs: true 时可以设置 top_logprobs 参数，取值范围为 [0, 20]。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<i32>,
    /// 调整指定 token 在模型输出内容中出现的概率，使模型生成的内容更加符合特定的偏好。
    /// logit_bias 字段接受一个 map 值，其中每个键为词表中的 token ID（使用 tokenization 接口获取），每个值为该 token 的偏差值，取值范围为 [-100, 100]。
    /// -1 会减少选择的可能性，1 会增加选择的可能性；-100 会完全禁止选择该 token，100 会导致仅可选择该 token。该参数的实际效果可能因模型而异。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String, i32>>,
    /// 模型可以调用的工具列表。目前，仅函数作为工具被支持。用这个来提供模型可能为其生成 JSON 输入的函数列表。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolParam>>,
}

#[derive(Serialize, Clone, Debug, Builder)]
pub struct ToolParam {
    /// 工具类型，当前仅支持 function
    pub r#type: String,
    /// 模型可以调用的工具列表。
    pub function: FunctionDefinition,
}

#[derive(Serialize, Clone, Debug, Builder)]
pub struct FunctionDefinition {
    /// 函数的名称, 比如：get_current_weather
    pub name: String,
    /// 对函数用途的描述，供模型判断何时以及如何调用该工具函数
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 函数请求参数，以 JSON Schema 格式描述。具体格式请参考 JSON Schema 文档
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

#[derive(Serialize, Clone, Debug, Builder)]
pub struct StreamOptionsParam {
    /// 是否包含本次请求的 token 用量统计信息
    /// false：不返回 token 用量信息
    /// true：在 data: [DONE] 消息之前返回一个额外的块，此块上的 usage 字段代表整个请求的 token 用量，choices 字段为空数组。所有其他块也将包含 usage 字段，但值为 null。
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "role")]
pub enum ChatCompletionMessage {
    /// System Message 系统消息
    System(SystemMessage),
    /// User Message 用户消息
    User(UserMessage),
    /// Assistant Message 对话助手消息
    Assistant(AssistantMessage),
    /// Tool Message 工具调用消息
    Tool(ToolMessage),
}

#[derive(Serialize, Clone, Debug)]
pub struct SystemMessage {
    /// 消息内容
    pub content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct UserMessage {
    /// 消息内容
    pub content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AssistantMessage {
    /// 消息内容
    pub content: Option<String>,
    pub tool_calls: Option<Vec<MessageToolCallParam>>,
}

#[derive(Serialize, Clone, Debug)]

pub struct MessageToolCallParam {
    /// 当前工具调用 ID
    id: String,
    /// 工具类型，当前仅支持function
    r#type: ToolCallType,
    /// 当前工具调用参数
    function: FunctionParam,
}

#[derive(Serialize, Clone, Debug)]
pub struct FunctionParam {
    /// 模型需要调用的函数名称
    name: String,
    /// 模型生成的用于调用函数的参数，JSON 格式。请注意，模型并不总是生成有效的 JSON，并且可能会虚构出一些您的函数参数规范中未定义的参数。在调用函数之前，请在您的代码中验证这些参数是否有效。
    arguments: String,
}

#[derive(Serialize, Clone, Debug, Default, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallType {
    #[default]
    Function,
}

#[derive(Serialize, Clone, Debug)]
pub struct ToolMessage {
    /// 消息内容
    content: String,
    /// 此消息所回应的工具调用 ID，当 role 为 tool 时必填
    tool_call_id: String,
}

////////////////////////////  Response  //////////////////////
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    /// 本次请求的唯一标识
    pub id: String,
    /// 本次请求实际使用的模型名称和版本
    pub model: String,
    /// 固定为 chat.completion(非流式)，固定为 chat.completion.chunk（流式）
    pub object: String,
    /// 本次请求创建时间的 Unix 时间戳（秒）
    pub created: i64,
    /// 本次请求的模型输出内容
    pub choices: Vec<Choice>,
    /// 本次请求的 tokens 用量
    pub usage: Option<Usage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    /// 当前元素在 choices 列表的索引
    pub index: usize,
    /// 模型停止生成 token 的原因。可能的值包括：
    /// stop：模型输出自然结束，或因命中请求参数 stop 中指定的字段而被截断
    /// length：模型输出因达到请求参数 max_token 指定的最大 token 数量而被截断
    /// content_filter：模型输出被内容审核拦截
    /// tool_calls：模型调用了工具
    pub finish_reason: String,
    /// 模型输出的内容
    pub message: Message,
    /// 当前内容的对数概率信息
    pub logprobs: Option<ChoiceLogprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    /// 固定为 assistant
    pub role: String,
    /// 模型生成的消息内容，content 与 tool_calls 字段二者至少有一个为非空
    pub content: Option<String>,
    /// 模型生成的消息内容，content 与 tool_calls 字段二者至少有一个为非空
    pub tool_calls: Option<Vec<MessageToolCall>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct MessageToolCall {
    /// 当前工具调用 ID
    pub id: String,
    /// 工具类型，当前仅支持function
    pub r#type: String,
    /// 当前工具调用参数
    pub function: Function,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Function {
    /// 模型需要调用的函数名称
    pub name: String,
    /// 模型生成的用于调用函数的参数，JSON 格式。请注意，模型并不总是生成有效的 JSON，并且可能会虚构出一些您的函数参数规范中未定义的参数。在调用函数之前，请在您的代码中验证这些参数是否有效。
    pub arguments: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceLogprobs {
    /// message列表中每个 content 元素中的 token 对数概率信息
    content: Vec<TokenLogprob>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLogprob {
    /// 当前 token
    token: String,
    /// 当前 token 的 UTF-8 值，格式为整数列表。当一个字符由多个 token 组成（表情符号或特殊字符等）时可以用于字符的编码和解码。如果 token 没有 UTF-8 值则为空。
    bytes: Vec<usize>,
    /// 当前 token 的对数概率
    logprob: f32,
    /// 在当前 token 位置最有可能的标记及其对数概率的列表。在一些情况下，返回的数量可能比请求参数 top_logprobs 指定的数量要少。
    top_logprobs: Option<Vec<TopLogprob>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TopLogprob {
    /// 当前 token
    token: String,
    /// 当前 token 的 UTF-8 值，格式为整数列表。当一个字符由多个 token 组成（表情符号或特殊字符等）时可以用于字符的编码和解码。如果 token 没有 UTF-8 值则为空。
    bytes: Vec<usize>,
    /// 当前 token 的对数概率
    logprob: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    /// 输入的 prompt token 数量
    pub prompt_tokens: usize,
    /// 模型生成的 token 数量
    pub completion_tokens: usize,
    /// 本次请求消耗的总 token 数量（输入 + 输出）
    pub total_tokens: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponseWrapper {
    /// 豆包返回多了一个data
    data: ChatCompletionChunkResponse,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunkResponse {
    /// 本次请求的唯一标识
    id: String,
    /// 本次请求实际使用的模型名称和版本
    model: String,
    /// 固定为 chat.completion(非流式)，固定为 chat.completion.chunk（流式）
    object: String,
    /// 本次请求创建时间的 Unix 时间戳（秒）
    created: i64,
    /// 本次请求的模型输出内容
    choices: Vec<StreamChoice>,
    /// 本次请求的 tokens 用量
    usage: Option<Usage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct StreamChoice {
    /// 当前元素在 choices 列表的索引
    index: usize,
    /// stop：模型输出自然结束，或因命中请求参数 stop 中指定的字段而被截断
    /// length：模型输出因达到请求参数 max_token 指定的最大 token 数量而被截断
    /// content_filter：模型输出被内容审核拦截
    /// tool_calls：模型调用了工具
    #[builder(setter(strip_option))]
    finish_reason: Option<String>,
    /// 模型输出的内容
    #[builder(setter(strip_option))]
    delta: Option<ChoiceDelta>,
    /// 当前内容的对数概率信息
    #[builder(setter(strip_option))]
    logprobs: Option<ChoiceLogprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct ChoiceDelta {
    /// 固定为 assistant
    role: String,
    /// 模型生成的消息内容，content 与 tool_calls 字段二者至少有一个为非空
    #[builder(setter(strip_option))]
    content: Option<String>,
    /// 模型生成的消息内容，content 与 tool_calls 字段二者至少有一个为非空
    #[builder(setter(strip_option))]
    tool_calls: Option<Vec<ChoiceDeltaToolCall>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceDeltaToolCall {
    /// 当前元素在 tool_calls 列表的索引
    index: usize,
    /// 当前工具调用 ID
    id: String,
    /// 工具类型，当前仅支持function
    r#type: String,
    /// 当前工具调用参数
    function: Function,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::MessageEvent;
    use tracing::info;
    use crate::LlmSdk;
    #[test]
    fn chat_completion_request_serialize_should_work() {
        let request = ChatCompletionRequestBuilder::default()
            .model("ep-20240817170913-w9q57".to_string())
            .messages(vec![
                ChatCompletionMessage::System(SystemMessage {
                    content: "你好".to_string(),
                }),
                ChatCompletionMessage::User(UserMessage {
                    content: "你是谁".to_string(),
                }),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        info!("json: {}", json);
        assert_eq!(
            json,
            r#"{"model":"ep-20240817170913-w9q57","messages":[{"role":"system","content":"你好"},{"role":"user","content":"你是谁"}]}"#
        );
    }

    #[tokio::test]
    async fn simple_chat_completion_should_work() -> Result<()> {
        let req = ChatCompletionRequestBuilder::default()
            .model("ep-20240929173453-xsp5x".to_string())
            .messages(vec![
                ChatCompletionMessage::System(SystemMessage {
                    content: "你好".to_string(),
                }),
                ChatCompletionMessage::User(UserMessage {
                    content: "你是谁".to_string(),
                }),
            ])
            .build()
            .unwrap();
        let sdk: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
        let res = sdk.chat_completion(&req).await?;
        //assert_eq!(res.model, ChatCompleteModel::Gpt3Turbo);
        assert_eq!(res.object, "chat.completion");
        //assert_eq!(res.choices.len(), 0);
        let choice = &res.choices[0];
        assert_eq!(choice.message.content.clone().unwrap(), "hello");

        //assert_eq!(choice.delta, "hello");
        //assert_eq!(choice.message.tool_calls.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn simple_chat_function_should_work() -> Result<()> {
        let req = ChatCompletionRequestBuilder::default()
            .model("ep-20250310183032-lljzt".to_string())
            .messages(vec![
                ChatCompletionMessage::System(SystemMessage {
                    content: "你好".to_string(),
                }),
                ChatCompletionMessage::User(UserMessage {
                    content: "打开空调".to_string(),
                }),
            ])
            .tools(vec![
               {
                   ToolParam {
                       r#type: "function".to_string(),
                       function: FunctionDefinition {
                           name: "open_aircondition".to_string(),
                           description: Some("打开空调".to_string()),
                           parameters: None,
                       } 
                   } 
               } 
            ])
            .build()
            .unwrap();
        let sdk: LlmSdk = LlmSdk::new(std::env::var("VOLC_API_KEY").unwrap());
        let res = sdk.chat_completion(&req).await?;
        assert_eq!(res.object, "chat.completion");
        let choice = &res.choices[0];
        assert_eq!(choice.message.content.clone().unwrap(), "hello");

        //assert_eq!(choice.delta, "hello");
        //assert_eq!(choice.message.tool_calls.len(), 0);
        Ok(())
    }


    #[tokio::test]
    async fn simple_chat_sequence_completion_should_work() -> Result<()> {
        let req = ChatCompletionRequestBuilder::default()
            .model("ep-20240817170913-w9q57".to_string())
            .messages(vec![
                ChatCompletionMessage::System(SystemMessage {
                    content: "你是我的私人助理，可以帮我记一些东西".to_string(),
                }),
                ChatCompletionMessage::User(UserMessage {
                    content: "记住我是你的主人，我叫孙庆鹏".to_string(),
                }),
            ])
            .build()
            .unwrap();
        let sdk: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
        let res = sdk.chat_completion(&req).await?;
        //assert_eq!(res.model, ChatCompleteModel::Gpt3Turbo);
        assert_eq!(res.object, "chat.completion");
        //assert_eq!(res.choices.len(), 0);
        //let choice = &res.choices[0];


        let req = ChatCompletionRequestBuilder::default()
        .model("ep-20240817170913-w9q57".to_string())
        .messages(vec![
            ChatCompletionMessage::User(UserMessage {
                content: "记住我是你的主人，我叫孙庆鹏".to_string(),
            }),
            ChatCompletionMessage::User(UserMessage {
                content: "你的主人是谁？".to_string(),
            }),
        ])
        .build()
        .unwrap();
        let sdk: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
        let res = sdk.chat_completion(&req).await?;
        let choice = &res.choices[0];
        assert_eq!(choice.message.content.clone().unwrap(), "hello");
        //assert_eq!(choice.delta, "hello");
        //assert_eq!(choice.message.tool_calls.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn simple_chat_completion_chunk_should_work() -> Result<()> {
        let req = ChatCompletionRequestBuilder::default()
            .model("ep-20240817170913-w9q57".to_string())
            .messages(vec![
                ChatCompletionMessage::System(SystemMessage {
                    content: "你好".to_string(),
                }),
                ChatCompletionMessage::User(UserMessage {
                    content: "你是谁".to_string(),
                }),
            ])
            .stream(true)
            .build()
            .unwrap();
        struct MyMessageEvent {}
        impl MessageEvent for MyMessageEvent {
            fn on_message(&self, chat_completion: &ChatCompletionChunkResponse) {
                info!("Chunk: {:?}\n", chat_completion);
            }

            fn on_end(&self) {
                info!("end");
            }
        }
        let sdk: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
        let result = sdk.chat_completion_stream(&req, &MyMessageEvent {}).await;
        match result {
            Ok(()) => (),
            Err(error) => {
                panic!("stream error: {:?}", error);
            }
        }
        //let hello = SDK.chat_completion_stream(&req).await?;
        //while let Some(chunk) = SDK.chat_completion_stream(&req).await? {
        //    info!("Chunk: {chunk:?}\n");
        //}
        assert_eq!("f", "hello");

        //assert_eq!(res.model, ChatCompleteModel::Gpt3Turbo);
        //assert_eq!(res.object, "chat.completion");
        //assert_eq!(res.choices.len(), 0);
        //let choice = &res.choices[0];
        //assert_eq!(choice.message.content.clone().unwrap(), "hello");
        //assert_eq!(choice.message.tool_calls.len(), 0);
        Ok(())
    }
}
