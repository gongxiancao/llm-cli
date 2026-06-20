use crate::config::Config;
use crate::types::*;
use anyhow::{Context, Result};
use base64::Engine;
use std::path::Path;

pub struct Provider {
    client: reqwest::Client,
    config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to create HTTP client");
        Self { client, config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    async fn post(&self, base: &str, path: &str, body: &serde_json::Value) -> Result<reqwest::Response> {
        let url = format!("{}{}", base.trim_end_matches('/'), path);
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .with_context(|| format!("Request to {url} failed"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&text) {
                anyhow::bail!("API error ({}): {}", status, err.error.message);
            }
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(resp)
    }

    async fn post_raw(&self, url: &str, body: &serde_json::Value) -> Result<reqwest::Response> {
        let resp = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .with_context(|| format!("Request to {url} failed"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&text) {
                anyhow::bail!("API error ({}): {}", status, err.error.message);
            }
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(resp)
    }

    pub async fn chat(
        &self,
        model: &str,
        system: Option<&str>,
        messages: Vec<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<ChatResponse> {
        let mut msgs = Vec::new();
        if let Some(sys) = system {
            msgs.push(Message {
                role: "system".to_string(),
                content: vec![ContentPart::Text {
                    text: sys.to_string(),
                }],
            });
        }
        for msg in messages {
            msgs.push(Message {
                role: "user".to_string(),
                content: vec![ContentPart::Text {
                    text: msg.to_string(),
                }],
            });
        }

        let req = ChatRequest {
            model: model.to_string(),
            messages: msgs,
            temperature,
            max_tokens,
        };

        let resp = self
            .post(&self.config.api_base, "/chat/completions", &serde_json::to_value(&req)?)
            .await?;
        let chat_resp: ChatResponse = resp
            .json()
            .await
            .context("Failed to parse chat response")?;
        Ok(chat_resp)
    }

    pub async fn vision(
        &self,
        model: &str,
        system: Option<&str>,
        text: &str,
        image_paths: &[String],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<ChatResponse> {
        let mut content_parts = Vec::new();
        content_parts.push(ContentPart::Text {
            text: text.to_string(),
        });

        for path in image_paths {
            let data_url = encode_image(path)?;
            content_parts.push(ContentPart::ImageUrl {
                image_url: ImageUrl { url: data_url },
            });
        }

        let mut msgs = Vec::new();
        if let Some(sys) = system {
            msgs.push(Message {
                role: "system".to_string(),
                content: vec![ContentPart::Text {
                    text: sys.to_string(),
                }],
            });
        }
        msgs.push(Message {
            role: "user".to_string(),
            content: content_parts,
        });

        let req = ChatRequest {
            model: model.to_string(),
            messages: msgs,
            temperature,
            max_tokens,
        };

        let resp = self
            .post(&self.config.api_base, "/chat/completions", &serde_json::to_value(&req)?)
            .await?;
        let chat_resp: ChatResponse = resp
            .json()
            .await
            .context("Failed to parse vision response")?;
        Ok(chat_resp)
    }

    pub async fn imagine(
        &self,
        model: &str,
        prompt: &str,
        n: Option<u32>,
        size: Option<&str>,
    ) -> Result<ImageGenerationResponse> {
        let req = ImageGenerationRequest {
            prompt: prompt.to_string(),
            model: model.to_string(),
            n,
            size: size.map(|s| s.to_string()),
        };

        let resp = self
            .post(&self.config.api_base, &self.config.image_api_path, &serde_json::to_value(&req)?)
            .await?;
        let img_resp: ImageGenerationResponse = resp
            .json()
            .await
            .context("Failed to parse image generation response")?;
        Ok(img_resp)
    }

    pub async fn imagine_dashscope(
        &self,
        model: &str,
        prompt: &str,
        n: Option<u32>,
        size: Option<&str>,
    ) -> Result<Vec<String>> {
        let dashscope_size = size.map(|s| s.replace('x', "*"));

        let content = vec![DashscopeContentPart::Text { text: prompt.to_string() }];
        let msg = DashscopeMessage {
            role: "user".to_string(),
            content,
        };
        let params = DashscopeParameters {
            n,
            size: dashscope_size,
        };
        let req = DashscopeRequest {
            model: model.to_string(),
            input: DashscopeInput { messages: vec![msg] },
            parameters: Some(params),
        };

        let resp = self
            .post_raw(&self.config.dashscope_endpoint, &serde_json::to_value(&req)?)
            .await?;
        let ds_resp: DashscopeResponse = resp
            .json()
            .await
            .context("Failed to parse dashscope image response")?;

        let mut urls = Vec::new();
        for choice in &ds_resp.output.choices {
            for part in &choice.message.content {
                if let DashscopeResponseContent::Image { image } = part {
                    urls.push(image.clone());
                }
            }
        }
        if let Some(usage) = &ds_resp.usage {
            eprintln!(
                "  Tokens: {} input + {} output = {} total",
                usage.input_tokens.unwrap_or(0),
                usage.output_tokens.unwrap_or(0),
                usage.total_tokens.unwrap_or(0)
            );
        }
        Ok(urls)
    }
}

fn encode_image(path: &str) -> Result<String> {
    let p = Path::new(path);
    let data = std::fs::read(p).with_context(|| format!("Failed to read image: {path}"))?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let mime = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/png",
    };
    Ok(format!("data:{mime};base64,{b64}"))
}
