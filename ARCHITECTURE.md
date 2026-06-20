# llm-cli - LLM CLI Tool Architecture

## Overview

llm-cli 是一个 Rust 驱动的命令行 LLM 工具，支持文本生成、图像理解和图像生成。采用 OpenAI 兼容 API 协议，可接入多个 provider。

## Development Convention

### Commit 策略
- 每个原子任务完成后立即提交
- 提交粒度：一个功能模块一次提交
- 提交信息用英文，格式：`type: short description`

### 阶段边界
- 严格遵守当前版本的开发范围，不做超前功能
- 如果在当前阶段提交前提出超出范围的需求，助手应提醒"将在后续版本中实现"

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2021) |
| CLI Framework | clap (derive) |
| HTTP Client | reqwest |
| Async Runtime | tokio |
| Serialization | serde + serde_json |
| Config Format | TOML |
| Error Handling | anyhow |

---

## Directory Structure

```
llm-cli/
├── Cargo.toml
└── src/
    ├── main.rs          # Entry point, command dispatch
    ├── cli.rs           # CLI argument definitions (clap derive)
    ├── config.rs        # TOML config file management (~/.config/llm-cli/config.toml)
    ├── provider.rs      # OpenAI-compatible API client
    └── types.rs         # Shared request/response types
```

---

## Architecture

```
User Terminal
     │
     ▼
┌─────────────────────────────────────────────┐
│  llm-cli (CLI)                               │
│                                              │
│  ┌──────────┐  ┌──────────┐  ┌────────────┐ │
│  │ chat     │  │ vision   │  │ imagine    │ │
│  │ 文本生成  │  │ 图像理解  │  │ 图像生成    │ │
│  └────┬─────┘  └────┬─────┘  └──────┬─────┘ │
│       │              │               │       │
│  ┌────▼──────────────▼───────────────▼─────┐ │
│  │          Provider (OpenAI Client)        │ │
│  │  ┌──────────┐  ┌──────────┐             │ │
│  │  │ chat()   │  │ vision() │  imagine()  │ │
│  │  └────┬─────┘  └────┬─────┘             │ │
│  └───────┼─────────────┼───────────────────┘ │
└──────────┼─────────────┼─────────────────────┘
           │             │
           ▼             ▼
    OpenAI Chat API    DALL-E API
   (text + vision)   (image generation)
```

---

## API Design

### Chat (文本生成)

```
POST /v1/chat/completions
  Request:
  {
    "model": "gpt-4o-mini",
    "messages": [
      { "role": "system", "content": [{ "type": "text", "text": "..." }] },
      { "role": "user", "content": [{ "type": "text", "text": "..." }] }
    ],
    "temperature": 0.7,
    "max_tokens": 2048
  }
  Response:
  {
    "choices": [{ "message": { "content": "..." } }],
    "usage": { "prompt_tokens": 10, "completion_tokens": 20, "total_tokens": 30 }
  }
```

### Vision (图像理解)

```
POST /v1/chat/completions
  Request:
  {
    "model": "gpt-4o",
    "messages": [{
      "role": "user",
      "content": [
        { "type": "text", "text": "描述这张图片" },
        { "type": "image_url", "image_url": { "url": "data:image/jpeg;base64,..." } }
      ]
    }]
  }
```

### Image Generation (图像生成)

```
POST /v1/images/generations
  Request:
  {
    "model": "dall-e-3",
    "prompt": "a cat wearing a hat",
    "n": 1,
    "size": "1024x1024"
  }
  Response:
  {
    "data": [{ "url": "https://..." }]
  }
```

---

## Data Flow

```
User Input (CLI arguments)
     │
     ▼
clap parse → cli::Commands
     │
     ├── Commands::Chat
     │    ├── config::load() → Config
     │    ├── Provider::chat(model, system, messages, temperature, max_tokens)
     │    │    └── POST /v1/chat/completions → OpenAI API
     │    └── Print response + token usage
     │
     ├── Commands::Vision
     │    ├── config::load() → Config
     │    ├── Read image files → base64 encode
     │    ├── Provider::vision(model, text, images, ...)
     │    │    └── POST /v1/chat/completions (multi-part content) → OpenAI API
     │    └── Print response + token usage
     │
     ├── Commands::Imagine
     │    ├── config::load() → Config
     │    ├── Provider::imagine(model, prompt, n, size)
     │    │    └── POST /v1/images/generations → DALL-E API
     │    └── Print image URL(s)
     │
     └── Commands::Config
          ├── config::show() → Print current config
          └── config::set(key, value) → Update + persist TOML
```

---

## Development Philosophy

优先构建最小可用原型，再基于原型逐步添加功能。每一步都产出可运行的版本，确保核心链路始终通畅。

```
MVP ──▶ v0.1 Chat ──▶ v0.2 Vision + Imagine ──▶ v0.3 Polish
```

每个版本都独立可用，互不阻塞。

---

## Development Phases

### v0.1 — CLI 骨架 + 文本生成 (Minimal Chat)

目标：跑通全链路 —— 用户在终端输入消息，LLM 回复。

#### 1a. 项目初始化
- [x] 初始化 Rust 项目，引入 clap + reqwest + tokio + serde + anyhow
- [x] `config` 模块：TOML 配置文件读写 (`~/.config/llm-cli/config.toml`)
- [x] `cli` 模块：clap derive 解析 `chat` / `vision` / `imagine` / `config` 子命令
- [x] `provider` 模块：基于 `reqwest` 的 OpenAI 兼容 API 客户端
- [x] `types` 模块：请求/响应的 serde 类型定义
- [x] `config set/show` 配置管理命令

#### 1b. 文本生成
- [x] `llm-cli chat <message>` 对话补全
- [x] 支持 `--model` / `--system` / `--temperature` / `--max-tokens` 参数
- [x] Token 用量统计输出到 stderr

> 交付物：一个能聊天的 CLI 工具。

---

### v0.2 — 图像理解 + 图像生成

目标：支持多模态能力。

#### 2a. 图像理解 (Vision)
- [x] `llm-cli vision <prompt> <images...>` 分析图片
- [x] 本地图片自动读取并 base64 编码
- [x] 支持 jpg / png / gif / webp 格式
- [x] 多图同时分析

#### 2b. 图像生成 (Imagine)
- [x] `llm-cli imagine <prompt>` 生成图片
- [x] 支持 `--model` / `--n` / `--size` 参数
- [x] 输出图片 URL

> 交付物：支持多模态的 CLI 工具。

---

### v0.3 — 打磨与增强

- [ ] 流式输出 (SSE streaming)
- [ ] 对话历史上下文 (multi-turn)
- [ ] 图片下载到本地 (`--output`)
- [ ] 多 Provider 抽象 trait
- [ ] 自动检测 image/vision 模型
- [ ] 颜色高亮输出

---

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
base64 = "0.22"
toml = "0.8"
```
