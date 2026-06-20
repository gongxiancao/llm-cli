mod cli;
mod config;
mod provider;
mod types;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigAction};
use provider::Provider;

fn print_response(text: &str) {
    println!("{}", text);
}

fn print_usage(usage: &types::Usage) {
    eprintln!(
        "  Tokens: {} prompt + {} completion = {} total",
        usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load()?;

    match cli.command {
        Commands::Chat {
            message,
            model,
            system,
            temperature,
            max_tokens,
        } => {
            if config.api_key.is_empty() {
                anyhow::bail!("API key not set. Run `llm-cli config set api_key <your-key>`");
            }
            let provider = Provider::new(config);
            let model = model.unwrap_or_else(|| provider.config().model.clone());
            let resp = provider
                .chat(
                    &model,
                    system.as_deref(),
                    vec![&message],
                    temperature.or(Some(provider.config().temperature)),
                    max_tokens.or(Some(provider.config().max_tokens)),
                )
                .await?;
            if let Some(choice) = resp.choices.first() {
                if let Some(content) = &choice.message.content {
                    print_response(content);
                }
            }
            if let Some(usage) = &resp.usage {
                print_usage(usage);
            }
        }
        Commands::Vision {
            prompt,
            images,
            model,
            system,
            temperature,
            max_tokens,
        } => {
            if config.api_key.is_empty() {
                anyhow::bail!("API key not set. Run `llm-cli config set api_key <your-key>`");
            }
            if images.is_empty() {
                anyhow::bail!("At least one image path is required");
            }
            for img in &images {
                if !std::path::Path::new(img).exists() {
                    anyhow::bail!("Image file not found: {img}");
                }
            }
            let provider = Provider::new(config);
            let model = model.unwrap_or_else(|| provider.config().vision_model.clone());
            let resp = provider
                .vision(
                    &model,
                    system.as_deref(),
                    &prompt,
                    &images,
                    temperature.or(Some(provider.config().temperature)),
                    max_tokens.or(Some(provider.config().max_tokens)),
                )
                .await?;
            if let Some(choice) = resp.choices.first() {
                if let Some(content) = &choice.message.content {
                    print_response(content);
                }
            }
            if let Some(usage) = &resp.usage {
                print_usage(usage);
            }
        }
        Commands::Imagine {
            prompt,
            model,
            n,
            size,
        } => {
            if config.api_key.is_empty() {
                anyhow::bail!("API key not set. Run `llm-cli config set api_key <your-key>`");
            }
            let provider = Provider::new(config);
            let model = model.unwrap_or_else(|| provider.config().image_model.clone());
            if !provider.config().dashscope_endpoint.is_empty() {
                let urls = provider
                    .imagine_dashscope(&model, &prompt, Some(n), size.as_deref())
                    .await?;
                for (i, url) in urls.iter().enumerate() {
                    println!("[{i}] {url}");
                }
                if urls.is_empty() {
                    eprintln!("No images returned");
                }
            } else {
                let resp = provider
                    .imagine(&model, &prompt, Some(n), size.as_deref())
                    .await?;
                for (i, img) in resp.data.iter().enumerate() {
                    if let Some(url) = &img.url {
                        println!("[{i}] {url}");
                    }
                    if let Some(b64) = &img.b64_json {
                        println!("[{i}] (base64, {} bytes)", b64.len());
                    }
                }
            }
        }
        Commands::Config { action } => match action {
            ConfigAction::Show => {
                let cfg = config::load()?;
                println!("api_base = {}", cfg.api_base);
                println!("model = {}", cfg.model);
                println!("vision_model = {}", cfg.vision_model);
                println!("image_model = {}", cfg.image_model);
                println!(
                    "api_key = {}",
                    if cfg.api_key.is_empty() {
                        "(not set)".to_string()
                    } else {
                        format!("{}...", &cfg.api_key[..cfg.api_key.len().min(8)])
                    }
                );
                println!("image_api_path = {}", cfg.image_api_path);
                println!(
                    "dashscope_endpoint = {}",
                    if cfg.dashscope_endpoint.is_empty() {
                        "(not set, using openai format)".to_string()
                    } else {
                        cfg.dashscope_endpoint.clone()
                    }
                );
                println!("temperature = {}", cfg.temperature);
                println!("max_tokens = {}", cfg.max_tokens);
            }
            ConfigAction::Set { key, value } => {
                config::set(&key, &value)?;
                println!("Set {key} = {value}");
                if key == "api_key" {
                    println!("API key saved to config file.");
                }
                println!("Config saved to: {}", config::config_path().display());
            }
        },
    }
    Ok(())
}
