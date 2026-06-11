//! System information collection.
//!
//! [`collect`] gathers raw values from [`sysinfo`] into a [`SystemInfo`]. The
//! data is deliberately kept raw (bytes as `u64`, durations as seconds, absent
//! values as `None`) so that formatting and rendering layers decide how to
//! present it.

use std::env;

use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

/// Raw, unformatted snapshot of the host system.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub host_name: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
    pub shell: Option<String>,
    pub uptime_secs: u64,
    pub cpu_model: Option<String>,
    pub cpu_count: usize,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
}

/// Memory and swap usage, in bytes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

/// A single mounted disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub file_system: String,
}

/// Collect a snapshot of the current system.
///
/// Uses targeted refreshes (memory + CPU list only) rather than `refresh_all`,
/// which avoids the expensive process scan.
pub fn collect() -> SystemInfo {
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing()),
    );

    let cpu_model = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim().to_string())
        .filter(|brand| !brand.is_empty());

    let disks = Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| DiskInfo {
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            total: disk.total_space(),
            available: disk.available_space(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
        })
        .collect();

    SystemInfo {
        host_name: System::host_name(),
        os_name: System::name(),
        os_version: System::os_version(),
        kernel_version: System::kernel_version(),
        shell: get_shell(),
        uptime_secs: System::uptime(),
        cpu_model,
        cpu_count: sys.cpus().len(),
        memory: MemoryInfo {
            total: sys.total_memory(),
            used: sys.used_memory(),
            swap_total: sys.total_swap(),
            swap_used: sys.used_swap(),
        },
        disks,
    }
}

/// The user's login shell from `$SHELL`, if set.
fn get_shell() -> Option<String> {
    env::var("SHELL").ok().filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_at_least_one_cpu() {
        let info = collect();
        assert!(info.cpu_count >= 1, "expected >=1 CPU, got {}", info.cpu_count);
    }

    #[test]
    fn reports_nonzero_total_memory() {
        let info = collect();
        assert!(info.memory.total > 0, "expected total memory > 0");
    }

    #[test]
    fn used_memory_does_not_exceed_total() {
        let info = collect();
        assert!(
            info.memory.used <= info.memory.total,
            "used {} exceeds total {}",
            info.memory.used,
            info.memory.total
        );
    }
}
