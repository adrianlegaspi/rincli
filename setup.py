import os

files = {
    'crates/rincli/src/commands/mod.rs': '''pub mod config;
pub mod doctor;
pub mod install;
pub mod logs;
pub mod models;
pub mod ps;
pub mod run;
pub mod runtime;
pub mod search;
pub mod stop;
''',
    'crates/rincli/src/commands/doctor.rs': '''use crate::errors::RincliError;
use crate::hardware;

pub fn run() -> Result<(), RincliError> {
    println!("RinCLI Doctor");
    hardware::print_doctor_info();
    Ok(())
}
''',
    'crates/rincli/src/commands/search.rs': '''use crate::errors::RincliError;

pub fn run(query: String, plain: bool, json: bool) -> Result<(), RincliError> {
    if json {
        println!(r#"{{"query": "{}", "results": []}}"#, query);
    } else if plain {
        println!("Search results for '{}'", query);
    } else {
        println!("Interactive search for '{}' (TUI stub)", query);
    }
    Ok(())
}
''',
    'crates/rincli/src/commands/install.rs': '''use crate::errors::RincliError;

pub fn run(model: String) -> Result<(), RincliError> {
    println!("Installing model {}", model);
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/models.rs': '''use crate::errors::RincliError;

pub fn run() -> Result<(), RincliError> {
    println!("Listing installed models");
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/run.rs': '''use crate::errors::RincliError;

pub fn run(model: String) -> Result<(), RincliError> {
    println!("Running model {}", model);
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/ps.rs': '''use crate::errors::RincliError;

pub fn run() -> Result<(), RincliError> {
    println!("Process status");
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/stop.rs': '''use crate::errors::RincliError;

pub fn run() -> Result<(), RincliError> {
    println!("Stopping active model");
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/logs.rs': '''use crate::errors::RincliError;

pub fn run() -> Result<(), RincliError> {
    println!("Showing logs");
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/commands/config.rs': '''use crate::errors::RincliError;

pub fn run_get() -> Result<(), RincliError> {
    println!("Getting config");
    Ok(())
}

pub fn run_set(key: String, value: String) -> Result<(), RincliError> {
    println!("Setting config {} = {}", key, value);
    Ok(())
}
''',
    'crates/rincli/src/commands/runtime.rs': '''use crate::errors::RincliError;

pub fn run_install() -> Result<(), RincliError> {
    println!("Installing runtime");
    Err(RincliError::NotImplemented)
}

pub fn run_doctor() -> Result<(), RincliError> {
    println!("Runtime doctor");
    Err(RincliError::NotImplemented)
}
''',
    'crates/rincli/src/hardware/mod.rs': '''pub mod linux;
pub mod macos;
pub mod scoring;
pub mod windows;

use sysinfo::System;

pub fn print_doctor_info() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("OS: {:?}", sys.name());
    println!("Total RAM: {} MB", sys.total_memory() / 1024 / 1024);
}
''',
    'crates/rincli/src/hardware/linux.rs': '',
    'crates/rincli/src/hardware/macos.rs': '',
    'crates/rincli/src/hardware/scoring.rs': '',
    'crates/rincli/src/hardware/windows.rs': '',
    'crates/rincli/src/config/mod.rs': '''pub mod paths;
pub mod schema;
''',
    'crates/rincli/src/config/paths.rs': '',
    'crates/rincli/src/config/schema.rs': '',
    'crates/rincli/src/db/mod.rs': '''pub mod migrations;
pub mod models;
''',
    'crates/rincli/src/db/migrations.rs': '',
    'crates/rincli/src/db/models.rs': '',
    'crates/rincli/src/registry/mod.rs': '''pub mod bundled;
pub mod remote;
pub mod hf;
pub mod schema;
pub mod scoring;
''',
    'crates/rincli/src/registry/bundled.rs': '',
    'crates/rincli/src/registry/remote.rs': '',
    'crates/rincli/src/registry/hf.rs': '',
    'crates/rincli/src/registry/schema.rs': '',
    'crates/rincli/src/registry/scoring.rs': '',
    'crates/rincli/src/download/mod.rs': '''pub mod resume;
pub mod verify;
''',
    'crates/rincli/src/download/resume.rs': '',
    'crates/rincli/src/download/verify.rs': '',
    'crates/rincli/src/runtime/mod.rs': '''pub mod llama_cpp;
pub mod manifest;
pub mod process;
''',
    'crates/rincli/src/runtime/llama_cpp.rs': '',
    'crates/rincli/src/runtime/manifest.rs': '',
    'crates/rincli/src/runtime/process.rs': '',
    'crates/rincli/src/tui/mod.rs': '''pub mod search;
''',
    'crates/rincli/src/tui/search.rs': '',
}

for filepath, content in files.items():
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, 'w') as f:
        f.write(content)
