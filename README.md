# llm-cli

Image understanding (vision) and image generation (imagine) CLI tool for AI agents. OpenAI-compatible, multi-provider.

## Install

```bash
curl -sfL https://github.com/gongxiancao/llm-cli/releases/latest/download/install.sh | sh
```

Or build from source:

```bash
cargo install --path .
```

## Quick Start

```bash
# Configure
llm-cli config set api_key "sk-xxx"

# Image understanding
llm-cli vision "描述这张图片" photo.jpg

# Image generation
llm-cli imagine "一只戴帽子的猫"

# Chat (bonus, agent should use its own LLM for text)
llm-cli chat "你好"
```

## Agent Skill

llm-cli is designed to be called by AI agents. See [SKILL.md](SKILL.md) for the complete skill definition — any agent with shell access can read it and use llm-cli.

## Commands

| Command | Description |
|---------|-------------|
| `vision <prompt> <images...>` | Analyze one or more images |
| `imagine <prompt>` | Generate an image from text |
| `chat <message>` | Text chat completion |
| `config set/show` | Manage configuration |

## Provider Switching

```bash
# 百炼 Dashscope
llm-cli config set api_base "https://dashscope.aliyuncs.com/compatible-mode/v1"
llm-cli config set vision_model "qwen-vl-plus"
llm-cli config set image_model "qwen-image-2.0-pro"
```

## License

MIT
