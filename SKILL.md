---
name: llm-cli
description: Image understanding and generation tool for AI agents
license: MIT
compatibility: opencode, claude, openclaw, nanobot, any agent with shell access
metadata:
  capabilities: vision, imagine, multimodal
  install_method: curl_pipe_sh
---

## What I do

llm-cli provides image understanding (vision) and image generation (imagine) via command line.
Agent's own LLM handles text reasoning; llm-cli covers the image side.

## Prerequisites

llm-cli binary must be in PATH. If not found, install it:

```bash
curl -sfL https://github.com/you/llm-cli/releases/latest/download/install.sh | sh
```

This detects OS/architecture and downloads the matching static binary to `~/.local/bin/`.

Verify installation:

```bash
llm-cli config show
```

Before first use, configure API key and model:

```bash
llm-cli config set api_key "your-api-key"
llm-cli config set vision_model "gpt-4o"
llm-cli config set image_model "dall-e-3"
```

See `llm-cli config show` for all configurable options.

## Commands

### vision — Image understanding

Analyze one or more images with a text prompt.

```bash
llm-cli vision "<prompt>" <image-path> [<image-path>...] [options]
```

Arguments:

| Arg | Required | Description |
|---|---|---|
| `<prompt>` | yes | Text prompt (e.g. "Describe this image") |
| `<images>` | yes | One or more image file paths |
| `--model` | no | Override vision model from config |
| `--system` | no | System prompt |
| `--temperature` | no | Temperature (0.0–2.0) |
| `--max-tokens` | no | Max output tokens |

Supported image formats: jpg, jpeg, png, gif, webp.

Examples:

```bash
llm-cli vision "描述这张图片" photo.jpg
llm-cli vision "这些图片有什么共同点" img1.jpg img2.png
llm-cli vision "图中是什么？" img.jpg --model qwen-vl-plus
```

### imagine — Image generation

Generate images from a text description.

```bash
llm-cli imagine "<prompt>" [options]
```

Arguments:

| Arg | Required | Description |
|---|---|---|
| `<prompt>` | yes | Image description |
| `--model` | no | Override image model from config |
| `--n` | no | Number of images (default: 1) |
| `--size` | no | Image size (e.g. "1024x1024", "1792x1024") |

Examples:

```bash
llm-cli imagine "一只戴帽子的猫"
llm-cli imagine "赛博朋克城市" --n 2 --size 1024x1024
llm-cli imagine "水墨风格山水" --model dall-e-3
```

### chat — Text generation (bonus)

Plain text chat completion. Use when agent's built-in LLM is not suitable (e.g. different provider needed).

```bash
llm-cli chat "<message>" [options]
```

## Output format

- **vision / chat**: Response text on stdout, token usage on stderr.
- **imagine**: Image URLs on stdout (one per line, prefixed with index `[0]`), errors on stderr.
- **exit code**: 0 on success, non-zero on error with error message on stderr.

## Configuration

Config file: `~/.config/llm-cli/config.toml`

| Key | Default | Description |
|---|---|---|
| `api_key` | — | API key (required) |
| `api_base` | `https://api.openai.com/v1` | API base URL |
| `model` | `gpt-4o-mini` | Default chat model |
| `vision_model` | `gpt-4o` | Vision model |
| `image_model` | `dall-e-3` | Image generation model |
| `temperature` | `0.7` | Default temperature |
| `max_tokens` | `2048` | Default max tokens |
| `dashscope_endpoint` | — | 百炼 multimodal endpoint (optional) |

Switch to different providers:

```bash
# 百炼 Dashscope
llm-cli config set api_base "https://dashscope.aliyuncs.com/compatible-mode/v1"
llm-cli config set model "qwen-plus"
llm-cli config set vision_model "qwen-vl-plus"
llm-cli config set image_model "qwen-image-2.0-pro"
llm-cli config set dashscope_endpoint "https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation"
```

## When to use

- **vision**: Agent receives an image and needs to describe, analyze, or extract information from it.
- **imagine**: Agent needs to generate an image from a text description.
- **chat**: Agent needs text generation from a different model/provider than its own.

## When NOT to use

- **Text-only reasoning/chat**: Use agent's built-in LLM instead. llm-cli is an image capability supplement.
