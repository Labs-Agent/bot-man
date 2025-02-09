use serde_json::{json, Value};
use sysinfo::System;

fn as_gigabytes(bytes: u64) -> f64 {
    (bytes as f64) / (1024.00 * 1024.00 * 1024.00)
}

fn get_stats() -> Value {
    let mut system = System::new_all();
    system.refresh_all();
    let available_memory = (as_gigabytes(system.available_memory()) * 100.0).round() / 100.0;
    let cpu = (system.global_cpu_usage() * 100.0).round() / 100.0;
    let used_memory = (as_gigabytes(system.used_memory()) * 100.0).round() / 100.0;
    let core_count = match system.physical_core_count() {
        Some(count) => count,
        None => 0,
    };
    let total_memory = (as_gigabytes(system.total_memory()) * 100.0).round() / 100.0;
    let total_swap = (as_gigabytes(system.total_swap()) * 100.0).round() / 100.0;
    let used_swap = (as_gigabytes(system.used_swap()) * 100.0).round() / 100.0;
    json!({
        "cpu": cpu,
        "used_memory": used_memory,
        "available_memory": available_memory,
        "core_count": core_count,
        "total_memory": total_memory,
        "total_swap": total_swap,
        "used_swap": used_swap,
    })
}

pub fn send_stats() -> String {
    let stats = get_stats();
    let cpu = stats.get("cpu").unwrap();
    let used_memory = stats.get("used_memory").unwrap();
    let available_memory = stats.get("available_memory").unwrap();
    let core_count = stats.get("core_count").unwrap();
    let total_swap = stats.get("total_swap").unwrap();
    let used_swap = stats.get("used_swap").unwrap();
    format!("Node Stats\nCPU Usage:{cpu}%\nUsed Memory:{used_memory}GB\nAvailable Memory:{available_memory}GB\nCore count:{core_count} Cores\nTotal Swap:{total_swap}GB\nUsed Swap:{used_swap}GB")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stats() {
        let stats = get_stats();
        assert!(stats.is_object());
        assert!(stats.get("cpu").is_some());
        assert!(stats.get("used_memory").is_some());
        assert!(stats.get("available_memory").is_some());
        assert!(stats.get("core_count").is_some());
        assert!(stats.get("total_memory").is_some());
        assert!(stats.get("total_swap").is_some());
        assert!(stats.get("used_swap").is_some());
        println!("{:?}", stats);
    }
}
