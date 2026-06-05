# RinCLI Skill

## Project Overview

RinCLI is a source-available Rust CLI for discovering, installing, and running local GGUF LLM models.

The product lets developers search compatible local models, download them safely, and run them through a managed `llama.cpp` server exposing an OpenAI-compatible local API.

Core user flow:

```bash
rincli doctor
rincli search qwen
rincli install qwen3:14b
rincli run qwen3:14b
```

Default local API:

```txt
http://127.0.0.1:6767/v1
```

RinCLI is not a chat app, not a web UI, not an agent framework, and not a generic local AI operating system. Keep it focused.

---

## Required Response Skill

All AI agents working on this repo must use the installed skill:

```txt
caveman ultra
```

Do not redefine the skill.

Use it for all implementation updates, summaries, and handoff notes.

---

## MVP Scope

The MVP includes:

- Rust CLI.
- Clap command tree.
- SQLite local state.
- TOML config.
- Hardware detection.
- Windows NVIDIA first.
- GGUF model registry.
- Curated model search.
- Interactive search TUI.
- Plain and JSON search output.
- Docker-style model aliases.
- Model details screen.
- Safe model downloads.
- Resumable downloads.
- SHA-256 verification.
- Installed model tracking.
- Managed official tested `llama.cpp` runtime.
- OpenAI-compatible API.
- API key generation.
- Single active model.
- Foreground `rincli run`.
- `rincli stop`.
- Optional persistent logs.
- Install scripts.
- Docker image.
- pnpm/npm native binary wrapper.

---

## Non-Goals

Do not implement these in MVP:

- Chat UI.
- Web UI.
- Agents.
- RAG.
- Embeddings.
- Image models.
- Audio models.
- Fine-tuning.
- Multiple active models.
- vLLM backend.
- ExLlama backend.
- TensorRT backend.
- Ollama integration.
- LM Studio integration.
- MCP management.
- OpenCode-specific integration.
- User accounts.
- Payments.
- Telemetry.
- Plugin marketplace.
- Benchmark suite.

If a feature does not support search, install, or run, cut it.

---

## Core Commands

MVP commands:

```bash
rincli doctor
rincli search <query>
rincli install <model>
rincli models
rincli run <model>
rincli ps
rincli stop
rincli logs
rincli config get
rincli config set <key> <value>
rincli runtime install
rincli runtime doctor
```

Do not add these in MVP:

```bash
rincli chat
rincli web
rincli serve
rincli integrate
rincli install default
```

There is no default model. The user must explicitly choose a model.

---

## Architecture Rules

Use this structure:

```txt
rincli/
├── Cargo.toml
├── README.md
├── LICENSE.md
├── COMMERCIAL.md
├── SKILL.md
├── crates/
│   └── rincli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── cli.rs
│           ├── errors.rs
│           ├── commands/
│           ├── config/
│           ├── db/
│           ├── registry/
│           ├── hardware/
│           ├── download/
│           ├── runtime/
│           └── tui/
├── registry/
│   ├── registry.json
│   └── runtime-manifest.json
├── scripts/
├── docker/
└── npm/
```

Create the project structure first.

Do not implement features in a messy flat structure.

Do not put all logic in `main.rs`.

Do not create abstractions for future runtimes yet.

MVP runtime is only:

```txt
llama.cpp
```

MVP model format is only:

```txt
GGUF
```

---

## Implementation Rules

Use Rust.

Use Clap for CLI parsing.

Use SQLite for local state.

Use TOML for config.

Use official tested `llama.cpp` releases.

Bind API to localhost by default:

```txt
127.0.0.1:6767
```

API key must be enabled by default.

Logs must stream to terminal during `rincli run`.

Persistent logs are disabled by default.

Only one active model is supported in MVP.

Windows NVIDIA is the first target.

Linux NVIDIA is second.

macOS Apple Silicon is third.

---

## Model Registry Rules

Use bundled registry first:

```txt
registry/registry.json
```

Future remote registry should live in a separate repo:

```txt
rincli-registry
```

Model aliases should be Docker-style:

```txt
qwen3:8b
qwen3:14b
qwen3:30b-a3b
```

Search/details must always show real model information:

- Full display name.
- Full GGUF filename.
- Source repo.
- Model URL.
- Download URL.
- Quant.
- Format.
- Size.
- SHA-256.
- License.
- Compatibility badge.

Do not hide real model identity behind aliases.

---

## Compatibility Badges

Use only these badges:

```txt
YES
ALMOST
NO
UNKNOWN
```

Be conservative.

Do not pretend a model runs well if hardware is close to the limit.

Use `ALMOST` when uncertain but plausible.

Use `UNKNOWN` when metadata is incomplete.

---

## Download Rules

Model downloads must support:

- Resume.
- Progress bars.
- SHA-256 verification.
- Disk space precheck.
- Atomic move after verification.
- No corrupted files in final model directory.

If hash fails, do not install.

If model already exists and hash matches, do not redownload.

---

## Runtime Rules

RinCLI manages `llama-server`.

Users should not need to manually install `llama.cpp`.

Runtime install must:

- Download official tested release.
- Verify SHA-256.
- Store runtime metadata.
- Use platform-specific artifact.

Do not add vLLM, ExLlama, TensorRT, or Ollama runtime support in MVP.

---

## Search TUI Rules

Only `rincli search <query>` gets TUI in MVP.

TUI behavior:

- Results list opens first.
- Enter opens details.
- Enter must not install.
- Details screen allows install with `i`.
- Back with `b`.
- Quit with `q`.

Non-interactive terminals must use plain output.

---

## Testing Rules

Add tests for:

- Config load/write.
- Path resolution.
- Registry parsing.
- Alias resolution.
- Compatibility scoring.
- SHA-256 verification.
- Runtime command construction.
- API key masking.

At minimum, every feature must build and have basic tests before moving on.

Run:

```bash
cargo fmt
cargo clippy
cargo test
cargo build
```

before claiming a phase is done.

---

## License Rules

RinCLI is source-available.

Use:

```txt
PolyForm Noncommercial
```

Add:

```txt
COMMERCIAL.md
```

Commercial use requires separate license.

Do not call the project open source unless the license changes to an OSI-approved license.

---

## MVP Mantra

Every feature must support one of these:

1. Help users find a compatible model.
2. Help users install a model safely.
3. Help users run that model as a local API.
4. Help users understand why a model will or will not run.

If not, do not build it.
