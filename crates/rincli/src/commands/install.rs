use crate::errors::RincliError;

pub fn run(model: String) -> Result<(), RincliError> {
    println!("Installing model {}", model);
    Err(RincliError::NotImplemented)
}
