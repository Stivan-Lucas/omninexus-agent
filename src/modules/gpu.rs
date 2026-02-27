use serde::Serialize;
use wmi::{COMLibrary, WMIConnection, Variant};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct GpuMetrics {
    pub name: String,
    pub vendor: String,
    pub memory_total_mb: u64,
    pub memory_used_mb: u64,
    pub load_percent: f32,
}

impl GpuMetrics {
    pub fn collect() -> Vec<Self> {
        let mut gpus = Vec::new();

        let com_lib = match COMLibrary::new() {
            Ok(lib) => lib,
            Err(_) => return gpus,
        };

        let wmi_con = match WMIConnection::new(com_lib) {
            Ok(con) => con,
            Err(_) => return gpus,
        };

        // 1. Dados Estáticos (Nome e Total VRAM)
        let static_results: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query("SELECT Name, AdapterCompatibility, AdapterRAM FROM Win32_VideoController")
            .unwrap_or_default();

        // 2. QUERY COMPLEXA: VRAM Utilizada
        // Somamos a DedicatedUsage de todas as instâncias ativas
        let vram_results: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query("SELECT DedicatedUsage FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUDedicatedMemory")
            .unwrap_or_default();

        let vram_used_bytes: u64 = vram_results.iter()
            .filter_map(|res| match res.get("DedicatedUsage") {
                Some(Variant::UI8(v)) => Some(*v),
                Some(Variant::UI4(v)) => Some(*v as u64),
                _ => None,
            }).sum();

        // 3. Query de Load (Engenharia 3D)
        let perf_results: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query("SELECT UtilizationPercentage FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine WHERE Name LIKE '%engtype_3D%'")
            .unwrap_or_default();

        let load_avg: f32 = if !perf_results.is_empty() {
            let sum: u64 = perf_results.iter()
                .filter_map(|res| match res.get("UtilizationPercentage") {
                    Some(Variant::UI8(v)) => Some(*v),
                    Some(Variant::UI4(v)) => Some(*v as u64),
                    _ => None,
                }).sum();
            sum as f32 / perf_results.len() as f32
        } else { 0.0 };

        for res in static_results {
            let name = match res.get("Name") {
                Some(Variant::String(s)) => s.clone(),
                _ => "Unknown GPU".into(),
            };

            let ram_total_bytes = match res.get("AdapterRAM") {
                Some(Variant::UI8(b)) => *b,
                Some(Variant::I8(b)) => *b as u64,
                Some(Variant::UI4(b)) => *b as u64,
                _ => 0,
            };

            gpus.push(Self {
                name,
                vendor: match res.get("AdapterCompatibility") {
                    Some(Variant::String(s)) => s.clone(),
                    _ => "Generic".into(),
                },
                memory_total_mb: ram_total_bytes / (1024 * 1024),
                // O WMI entrega DedicatedUsage em Bytes
                memory_used_mb: vram_used_bytes / (1024 * 1024),
                load_percent: load_avg,
            });
        }

        gpus
    }
}
