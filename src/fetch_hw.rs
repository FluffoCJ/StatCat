use libc::statvfs;
use std::ffi::CString;
use std::fs::{self, read_to_string};

pub fn get_cpu() -> Option<String> {
    if let Ok(cpuinfo) = read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                return line.split(':').nth(1).map(|s| s.trim().to_string());
            }
        }
    }
    None
}

pub struct Storage {
    pub total_storage: u64,
    pub free_storage: u64,
    pub used_storage: u64,
}

pub fn get_storage() -> Storage {
    let path = CString::new("/").unwrap();

    let mut stats: libc::statvfs = unsafe { std::mem::zeroed() };

    let result = unsafe { statvfs(path.as_ptr(), &mut stats) };

    let block_size = stats.f_frsize as u64;
    let total_storage = block_size * stats.f_blocks as u64;
    let free_storage = block_size * stats.f_bfree as u64;
    let used_storage = total_storage - free_storage;

    let bytes_to_gb = 1_073_741_824;
    Storage {
        total_storage: total_storage / bytes_to_gb,
        free_storage: free_storage / bytes_to_gb,
        used_storage: used_storage / bytes_to_gb,
    }
}

pub struct Memory {
    pub total_kb: u64,
    pub free_kb: u64,
    pub used_kb: u64,
    pub total_mb: f64,
    pub free_mb: f64,
    pub used_mb: f64,
    pub total_gb: f64,
    pub free_gb: f64,
    pub used_gb: f64,
}

pub fn get_memory() -> Memory {
    let contents = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");

    let mut total = 0;
    let mut free = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("MemFree:") {
            free = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("Buffers:") {
            buffers = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("Cached:") {
            cached = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
    }

    let used = total - free - buffers - cached;

    let total_mb = total as f64 / 1024.0;
    let free_mb = free as f64 / 1024.0;
    let used_mb = used as f64 / 1024.0;

    let total_gb = total_mb / 1024.0;
    let free_gb = free_mb / 1024.0;
    let used_gb = used_mb / 1024.0;

    Memory {
        total_kb: total,
        free_kb: free,
        used_kb: used,
        total_mb: (total_mb * 10.0).round() / 10.0,
        free_mb: (free_mb * 10.0).round() / 10.0,
        used_mb: (used_mb * 10.0).round() / 10.0,
        total_gb: (total_gb * 10.0).round() / 10.0,
        free_gb: (free_gb * 10.0).round() / 10.0,
        used_gb: (used_gb * 10.0).round() / 10.0,
    }
}
