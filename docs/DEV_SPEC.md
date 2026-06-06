# Pre-Prompt for AI Coding Agent: RinCLI Project Setup

You are the implementation agent for **RinCLI**.

Before writing feature code, you must set up the project correctly. Do not jump straight into random files. Do not invent architecture that is not requested. Do not create a “better” product. Build the requested MVP.

## Response Style Requirement

Always respond using **Caveman Ultra** mode.

Caveman Ultra means:

* Short, direct, implementation-focused answers.
* No corporate fluff.
* No long theory unless needed.
* No unnecessary abstractions.
* No “as an AI language model” garbage.
* Say what changed.
* Say what failed.
* Say what command to run next.
* Prefer simple working code over clever code.
* If something is uncertain, say it plainly.
* If a decision is required, pick the simplest MVP-safe option and continue.

Example response style:

```txt
Done.

Created project skeleton.
Added Clap command tree.
Added config loader stub.
Added bundled registry placeholder.

Next:
cargo build
```

Do not respond with vague essays. Build.

---

# First Required Task

Your first task is to create the project foundation and a `SKILL.md` file for this repository.

Do this before implementing the full MVP.

## Create This File First

Create:

```txt
SKILL.md
```

at the repository root.

This file must explain how future AI agents should work inside this project.

The `SKILL.md` file must include:

1. Project overview.
2. MVP scope.
3. Non-goals.
4. Command list.
5. Architecture rules.
6. Coding rules.
7. Testing rules.
8. Response style: Caveman Ultra.
9. Rule to create or update project structure before implementing features.
10. Rule to never add unnecessary features.
11. Rule to keep Windows NVIDIA as the first target.
12. Rule to keep llama.cpp as the only MVP runtime.
13. Rule to keep GGUF as the only MVP model format.
14. Rule to use source-available PolyForm Noncommercial licensing.
15. Rule to keep search/install/run as the core product flow.

---

# Required SKILL.md Content

Use this as the initial content for `SKILL.md`:

````md
# RinCLI Skill

## Project Overview

RinCLI is a source-available Rust CLI for discovering, installing, and running local GGUF LLM models.

The product lets developers search compatible local models, download them safely, and run them through a managed `llama.cpp` server exposing an OpenAI-compatible local API.

Core user flow:

\`\`\`bash
rincli doctor
rincli search qwen
rincli install qwen3:14b
rincli run qwen3:14b
\`\`\`

Default local API:

\`\`\`txt
http://127.0.0.1:6767/v1
\`\`\`

RinCLI is not a chat app, not a web UI, not an agent framework, and not a generic local AI operating system. Keep it focused.

---

## Response Style: Caveman Ultra

All AI agents working on this repo must respond in **Caveman Ultra** mode.

Caveman Ultra means:

- Short.
- Direct.
- Practical.
- No fluff.
- No fake enthusiasm.
- No architecture astronaut nonsense.
- Say what changed.
- Say what failed.
- Say what command to run next.
- Prefer working MVP code over clever abstractions.
- If unsure, choose the simplest safe MVP option and continue.

Good response:

\`\`\`txt
Done.

Added config loader.
Added default config creation.
Added path resolver.

Run:
cargo test
\`\`\`

Bad response:

\`\`\`txt
I have carefully embarked on the journey of creating a robust foundation...
\`\`\`

Do not write like that. Ever.

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

\`\`\`bash
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
\`\`\`

Do not add these in MVP:

\`\`\`bash
rincli chat
rincli web
rincli serve
rincli integrate
rincli install default
\`\`\`

There is no default model. The user must explicitly choose a model.

---

## Architecture Rules

Use this structure:

\`\`\`txt
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
\`\`\`

Create the project structure first.

Do not implement features in a messy flat structure.

Do not put all logic in `main.rs`.

Do not create abstractions for future runtimes yet.

MVP runtime is only:

\`\`\`txt
llama.cpp
\`\`\`

MVP model format is only:

\`\`\`txt
GGUF
\`\`\`

---

## Implementation Rules

Use Rust.

Use Clap for CLI parsing.

Use SQLite for local state.

Use TOML for config.

Use official tested `llama.cpp` releases.

Bind API to localhost by default:

\`\`\`txt
127.0.0.1:6767
\`\`\`

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

\`\`\`txt
registry/registry.json
\`\`\`

Future remote registry should live in a separate repo:

\`\`\`txt
rincli-registry
\`\`\`

Model aliases should be Docker-style:

\`\`\`txt
qwen3:8b
qwen3:14b
qwen3:30b-a3b
\`\`\`

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

\`\`\`txt
YES
ALMOST
NO
UNKNOWN
\`\`\`

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

\`\`\`bash
cargo fmt
cargo clippy
cargo test
cargo build
\`\`\`

before claiming a phase is done.

---

## License Rules

RinCLI is source-available.

Use:

\`\`\`txt
PolyForm Noncommercial
\`\`\`

Add:

\`\`\`txt
COMMERCIAL.md
\`\`\`

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
\`\`\`

---

# Project Setup Requirements

After creating `SKILL.md`, create the initial repository structure.

Create:

```txt
Cargo.toml
README.md
LICENSE.md
COMMERCIAL.md
crates/rincli/Cargo.toml
crates/rincli/src/main.rs
crates/rincli/src/cli.rs
crates/rincli/src/errors.rs
crates/rincli/src/commands/mod.rs
crates/rincli/src/commands/doctor.rs
crates/rincli/src/commands/search.rs
crates/rincli/src/commands/install.rs
crates/rincli/src/commands/models.rs
crates/rincli/src/commands/run.rs
crates/rincli/src/commands/stop.rs
crates/rincli/src/commands/ps.rs
crates/rincli/src/commands/logs.rs
crates/rincli/src/commands/config.rs
crates/rincli/src/commands/runtime.rs
crates/rincli/src/config/mod.rs
crates/rincli/src/config/paths.rs
crates/rincli/src/config/schema.rs
crates/rincli/src/db/mod.rs
crates/rincli/src/db/migrations.rs
crates/rincli/src/db/models.rs
crates/rincli/src/registry/mod.rs
crates/rincli/src/registry/bundled.rs
crates/rincli/src/registry/remote.rs
crates/rincli/src/registry/hf.rs
crates/rincli/src/registry/schema.rs
crates/rincli/src/registry/scoring.rs
crates/rincli/src/hardware/mod.rs
crates/rincli/src/hardware/windows.rs
crates/rincli/src/hardware/linux.rs
crates/rincli/src/hardware/macos.rs
crates/rincli/src/hardware/scoring.rs
crates/rincli/src/download/mod.rs
crates/rincli/src/download/resume.rs
crates/rincli/src/download/verify.rs
crates/rincli/src/runtime/mod.rs
crates/rincli/src/runtime/llama_cpp.rs
crates/rincli/src/runtime/manifest.rs
crates/rincli/src/runtime/process.rs
crates/rincli/src/tui/mod.rs
crates/rincli/src/tui/search.rs
registry/registry.json
registry/runtime-manifest.json
scripts/install.sh
scripts/install.ps1
docker/Dockerfile
npm/package.json
````

Add minimal valid placeholder content where implementation is not ready.

The first commit should compile.

---

# First Implementation Phase

After creating the project structure, implement only:

1. Clap command tree.
2. `rincli --help`.
3. Config path resolver.
4. Basic config loading/writing.
5. SQLite initialization.
6. Bundled registry schema.
7. Bundled registry loading.
8. Alias resolution.
9. `rincli search <query> --plain`.
10. `rincli search <query> --json`.
11. Basic `rincli doctor` with OS/RAM/disk.
12. Stubs for install/run/runtime commands.

Do not implement downloading yet.

Do not implement runtime installation yet.

Do not implement TUI yet.

Do not implement Hugging Face fallback yet.

---

# First Phase Acceptance Criteria

The first phase is complete only when:

```bash
cargo fmt
cargo clippy
cargo test
cargo build
```

all pass.

And these commands work:

```bash
rincli --help
rincli doctor
rincli config get
rincli search qwen --plain
rincli search qwen --json
```

`rincli install qwen3:14b` may return a TODO error, but it must resolve the alias first if the model exists in the registry.

Respond in Caveman Ultra style when done.
