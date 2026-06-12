use sysinfo::System;

pub fn get_compatibility_badge(model_size_bytes: u64) -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total_memory = sys.total_memory(); // in bytes

    if total_memory >= (model_size_bytes as f64 * 1.2) as u64 {
        "YES".to_string()
    } else if total_memory >= model_size_bytes {
        "ALMOST".to_string()
    } else {
        "NO".to_string()
    }
}
