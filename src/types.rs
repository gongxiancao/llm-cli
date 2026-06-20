use serde::{Deserialize, Serialize};

// ---- OpenAI-compatible types ----

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: MessageContent,
    #[allow(dead_code)]
    #[serde(default)]
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ---- OpenAI Image Generation ----

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageGenerationResponse {
    pub data: Vec<ImageData>,
}

#[derive(Debug, Deserialize)]
pub struct ImageData {
    pub url: Option<String>,
    pub b64_json: Option<String>,
}

// ---- Dashscope (百炼) types ----

#[derive(Debug, Serialize)]
pub struct DashscopeRequest {
    pub model: String,
    pub input: DashscopeInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<DashscopeParameters>,
}

#[derive(Debug, Serialize)]
pub struct DashscopeInput {
    pub messages: Vec<DashscopeMessage>,
}

#[derive(Debug, Serialize)]
pub struct DashscopeMessage {
    pub role: String,
    pub content: Vec<DashscopeContentPart>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DashscopeContentPart {
    Text { text: String },
    #[allow(dead_code)]
    Image { image: String },
}

#[derive(Debug, Serialize)]
pub struct DashscopeParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DashscopeResponse {
    pub output: DashscopeOutput,
    pub usage: Option<DashscopeUsage>,
}

#[derive(Debug, Deserialize)]
pub struct DashscopeOutput {
    pub choices: Vec<DashscopeChoice>,
}

#[derive(Debug, Deserialize)]
pub struct DashscopeChoice {
    pub message: DashscopeResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct DashscopeResponseMessage {
    pub content: Vec<DashscopeResponseContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DashscopeResponseContent {
    #[allow(dead_code)]
    Text { text: String },
    Image { image: String },
}

#[derive(Debug, Deserialize)]
pub struct DashscopeUsage {
    pub total_tokens: Option<u32>,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}

// ---- Common ----

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    #[allow(dead_code)]
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
