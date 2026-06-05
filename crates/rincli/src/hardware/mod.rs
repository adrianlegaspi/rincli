pub mod linux;
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
