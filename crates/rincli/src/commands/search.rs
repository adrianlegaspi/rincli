use crate::errors::RincliError;
use crate::registry::hf::search_hf;
use crate::registry::scoring::get_compatibility_badge;
use colored::Colorize;

fn make_terminal_link(url: &str, text: &str, plain: bool) -> String {
    if plain {
        text.to_string()
    } else {
        format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
    }
}

pub fn run(query: String, plain: bool, json: bool) -> Result<(), RincliError> {
    if plain {
        colored::control::set_override(false);
    }

    let filtered = search_hf(&query)?;

    if json {
        let mut results_json = Vec::new();
        for m in filtered {
            let badge = get_compatibility_badge(m.size_bytes);
            let mut val = serde_json::to_value(&m)?;
            if let serde_json::Value::Object(ref mut map) = val {
                map.insert("compatibility_badge".to_string(), serde_json::Value::String(badge));
            }
            results_json.push(val);
        }
        let output = serde_json::json!({
            "query": query,
            "results": results_json
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Search results for '{}':", query);
        if filtered.is_empty() {
            println!("  No models found.");
        } else {
            println!();
            for m in &filtered {
                let badge = get_compatibility_badge(m.size_bytes);
                let size_gb = m.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                
                let compatibility_str = match badge.as_str() {
                    "YES" => "Compatible".green().bold(),
                    "ALMOST" => "Borderline RAM".yellow().bold(),
                    _ => "Incompatible (Requires more RAM)".red().bold(),
                };

                let alias_colored = m.alias.cyan().to_string();
                let alias_link = make_terminal_link(&m.model_url, &alias_colored, plain);

                println!(
                    "{} {} ({})",
                    "●".blue(),
                    m.display_name.bold(),
                    alias_link
                );
                println!(
                    "  {:15} {}",
                    "Compatibility:".dimmed(),
                    compatibility_str
                );
                println!(
                    "  {:15} {:.2} GB | {} | {}",
                    "Details:".dimmed(),
                    size_gb,
                    m.quant,
                    m.format
                );
                println!(
                    "  {:15} {}",
                    "Repository:".dimmed(),
                    m.source_repo
                );
                println!(
                    "  {:15} {}",
                    "Install:".dimmed(),
                    format!("rincli install {}", m.alias).bright_magenta().bold()
                );
                println!();
            }

            // Interactive install prompt (only when not in plain/JSON mode)
            if !plain {
                use std::io::{stdin, stdout, Write};
                print!("{}", "Would you like to install one of these models? (y/N): ".bright_green().bold());
                let _ = stdout().flush();
                let mut input = String::new();
                if stdin().read_line(&mut input).is_ok() {
                    let choice = input.trim().to_lowercase();
                    if choice == "y" || choice == "yes" {
                        println!("\n{}", "Select a model to install:".bold());
                        for (i, m) in filtered.iter().enumerate() {
                            println!("  [{}] {}", i + 1, m.alias.cyan());
                        }
                        print!("\n{}", "Enter number: ".bright_green().bold());
                        let _ = stdout().flush();
                        let mut selection = String::new();
                        if stdin().read_line(&mut selection).is_ok() {
                            if let Ok(num) = selection.trim().parse::<usize>() {
                                if num > 0 && num <= filtered.len() {
                                    let selected = &filtered[num - 1];
                                    println!();
                                    let _ = crate::commands::install::run(selected.alias.clone());
                                } else {
                                    println!("{}", "Invalid number.".red());
                                }
                            } else {
                                println!("{}", "Invalid input.".red());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

