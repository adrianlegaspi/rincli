use crate::errors::RincliError;

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
