use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
    #[serde(default = "default_image_model")]
    pub image_model: String,
    #[serde(default = "default_vision_model")]
    pub vision_model: String,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_image_model() -> String {
    "dall-e-3".to_string()
}

fn default_vision_model() -> String {
    "gpt-4o".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    2048
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
            image_model: default_image_model(),
            vision_model: default_vision_model(),
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("llm-cli")
}

fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        let config = Config::default();
        save(&config)?;
        return Ok(config);
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config at {:?}", path))?;
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config at {:?}", path))?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create config dir {:?}", dir))?;
    let content = toml::to_string_pretty(config)?;
    std::fs::write(config_path(), &content)
        .with_context(|| format!("Failed to write config at {:?}", config_path()))?;
    Ok(())
}

pub fn set(key: &str, value: &str) -> Result<Config> {
    let mut config = load().unwrap_or_default();
    match key {
        "api_base" => config.api_base = value.to_string(),
        "api_key" => config.api_key = value.to_string(),
        "model" => config.model = value.to_string(),
        "image_model" => config.image_model = value.to_string(),
        "vision_model" => config.vision_model = value.to_string(),
        "temperature" => {
            config.temperature = value.parse().context("temperature must be a float")?
        }
        "max_tokens" => {
            config.max_tokens = value.parse().context("max_tokens must be an integer")?
        }
        _ => anyhow::bail!("Unknown config key: {key}. Valid keys: api_base, api_key, model, image_model, vision_model, temperature, max_tokens"),
    }
    save(&config)?;
    Ok(config)
}
