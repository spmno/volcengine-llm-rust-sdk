use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug, Builder)]
pub struct EmbeddingsRequest {
    /// 您创建的推理接入点 ID, ep-202406040*****-*****
    model: String,
    /// 需要向量化的内容列表。输入字符需要满足：
    /// 1. 不得超过模型的最大输入 token 数（对于 doubao-embdding 模型为 4096）
    /// 2. 不能为空列表，列表的每个成员不能为空字符串
    /// 3. 单条文本以 utf-8 编码，长度不超过 100,000 字节
    /// 4. 文本数量不超过 256 条
    input: Vec<String>,
    /// embedding 返回的格式，当前支持 float或base64
    encoding_format: String
}

#[derive(Deserialize, Clone, Debug, Builder)]
pub struct EmbeddingsResponse {
    /// 本次请求的唯一标识
    id: String,
    /// 本次请求实际使用的模型名称和版本
    model: String,
    /// 本次请求创建时间的 Unix 时间戳（秒）
    created: u32,
    /// 固定为 list
    object: String,
    /// 本次请求的算法输出内容
    data: Embedding,
    /// 本次请求的 tokens 用量
    usage: Usage
}

#[derive(Deserialize, Clone, Debug, Builder)]
pub struct Embedding {
    /// 向量的序号，与请求参数 input 列表中的内容顺序对应
    index: u32,
    /// 对应内容的向量化结果,
    embedding: Vec<f32>,
    /// 固定为 embedding
    object: String
}

#[derive(Deserialize, Clone, Debug, Builder)]
pub struct Usage {
    /// 输入的 prompt token 数量
    prompt_tokens: u32,
    /// 本次请求消耗的总 token 数量（输入 + 输出)
    total_tokens: u32
}