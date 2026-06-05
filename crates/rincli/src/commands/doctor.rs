use crate::errors::RincliError;
use crate::hardware;

pub fn run() -> Result<(), RincliError> {
    println!("RinCLI Doctor");
    hardware::print_doctor_info();
    Ok(())
}
