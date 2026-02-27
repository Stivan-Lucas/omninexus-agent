use serde::Serialize;
use sysinfo::{Disks};

#[derive(Serialize, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space_gb: u64,
    pub available_space_gb: u64,
    pub used_space_gb: u64,
    pub usage_percent: f32,
    pub is_removable: bool,
}

pub struct DiskMetrics;

impl DiskMetrics {
    pub fn collect() -> Vec<DiskInfo> {
        // Na v0.33, criamos a lista de discos e já a atualizamos
        let disks = Disks::new_with_refreshed_list();
        let mut disk_list = Vec::new();

        for disk in &disks {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;

            let usage_percent = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };

            disk_list.push(DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                // Conversão de Bytes para GB
                total_space_gb: total / 1024 / 1024 / 1024,
                available_space_gb: available / 1024 / 1024 / 1024,
                used_space_gb: used / 1024 / 1024 / 1024,
                usage_percent,
                is_removable: disk.is_removable(),
            });
        }

        disk_list
    }
}
