use crate::errors::RincliError;

pub fn run(model: String) -> Result<(), RincliError> {
    println!("Running model {}", model);
    Err(RincliError::NotImplemented)
}
