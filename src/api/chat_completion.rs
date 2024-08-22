use crate::IntoRequest;
use derive_builder::Builder;
use reqwest::{Client, RequestBuilder};
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

#[derive(Serialize, Clone, Debug)]
pub struct ToolParam {
    /// 工具类型，当前仅支持 function
    name: String,
    /// 模型可以调用的工具列表。
    function: FunctionDefinition,
}

#[derive(Serialize, Clone, Debug)]
pub struct FunctionDefinition {
    /// 函数的名称, 比如：get_current_weather
    name: String,
    /// 对函数用途的描述，供模型判断何时以及如何调用该工具函数
    description: String,
    /// 函数请求参数，以 JSON Schema 格式描述。具体格式请参考 JSON Schema 文档
    parameters: serde_json::Value,

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
    content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct UserMessage { 
    /// 消息内容
    content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AssistantMessage {
    /// 消息内容
    content: Option<String>,
    tool_calls: Option<Vec<MessageToolCallParam>>,
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
    Function
}

#[derive(Serialize, Clone, Debug)]
pub struct ToolMessage {
    /// 消息内容
    content: String,
    /// 此消息所回应的工具调用 ID，当 role 为 tool 时必填
    tool_call_id: String,
}

impl IntoRequest for ChatCompletionRequest {
    fn into_request(self, base_url: &str, client: Client) -> RequestBuilder {
        let url = format!("{}/chat/completions", base_url);
        client.post(url).json(&self)
    }
}


////////////////////////////  Response  //////////////////////

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {

}


#[cfg(test)]
mod tests {
    use super::*;

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
                })
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        println!("json: {}", json);
        assert_eq!(json, r#"{"model":"ep-20240817170913-w9q57","messages":[{"role":"system","content":"你好"},{"role":"user","content":"你是谁"}]}"#);
    }

}


