use crate::errors::RincliError;

pub fn run_get() -> Result<(), RincliError> {
    println!("Getting config");
    Ok(())
}

pub fn run_set(key: String, value: String) -> Result<(), RincliError> {
    println!("Setting config {} = {}", key, value);
    Ok(())
}
