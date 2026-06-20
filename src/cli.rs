use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "llm-cli", version, about = "A CLI tool for LLM interactions")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Send a text chat message
    Chat {
        /// The message to send
        message: String,
        /// Model to use (overrides config)
        #[arg(long)]
        model: Option<String>,
        /// System prompt
        #[arg(long)]
        system: Option<String>,
        /// Temperature (0.0 - 2.0)
        #[arg(long)]
        temperature: Option<f32>,
        /// Max tokens in response
        #[arg(long)]
        max_tokens: Option<u32>,
    },
    /// Analyze images with a text prompt
    Vision {
        /// Text prompt
        prompt: String,
        /// Image file paths
        images: Vec<String>,
        /// Model to use (overrides config)
        #[arg(long)]
        model: Option<String>,
        /// System prompt
        #[arg(long)]
        system: Option<String>,
        /// Temperature (0.0 - 2.0)
        #[arg(long)]
        temperature: Option<f32>,
        /// Max tokens in response
        #[arg(long)]
        max_tokens: Option<u32>,
    },
    /// Generate an image from a prompt
    Imagine {
        /// Text description of the image
        prompt: String,
        /// Model to use (overrides config)
        #[arg(long)]
        model: Option<String>,
        /// Number of images to generate
        #[arg(long, default_value = "1")]
        n: u32,
        /// Image size (e.g. 1024x1024)
        #[arg(long)]
        size: Option<String>,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a config value: api_base|api_key|model|image_model|vision_model|temperature|max_tokens
    Set {
        key: String,
        value: String,
    },
}
