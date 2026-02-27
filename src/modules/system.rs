use serde::Serialize;
use sysinfo::System;
use wmi::{COMLibrary, WMIConnection, Variant};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct SystemMetrics {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub uptime_seconds: u64,
    pub uuid: String,           // ID Único da BIOS/Sistema
    pub serial_number: String,  // Serial da Placa Mãe
}

impl SystemMetrics {
    pub fn collect(_sys: &System) -> Self {
        let mut uuid = "Unknown".to_string();
        let mut serial = "Unknown".to_string();

        // Consulta WMI para Identificadores Únicos
        if let Ok(com_lib) = COMLibrary::new() {
            if let Ok(wmi_con) = WMIConnection::new(com_lib) {
                // 1. Pega o UUID do Sistema
                let uuid_res: Vec<HashMap<String, Variant>> = wmi_con
                    .raw_query("SELECT UUID FROM Win32_ComputerSystemProduct")
                    .unwrap_or_default();

                if let Some(res) = uuid_res.first() {
                    if let Some(Variant::String(s)) = res.get("UUID") {
                        uuid = s.clone();
                    }
                }

                // 2. Pega o Serial da Placa Mãe (BaseBoard)
                let serial_res: Vec<HashMap<String, Variant>> = wmi_con
                    .raw_query("SELECT SerialNumber FROM Win32_BaseBoard")
                    .unwrap_or_default();

                if let Some(res) = serial_res.first() {
                    if let Some(Variant::String(s)) = res.get("SerialNumber") {
                        serial = s.trim().to_string();
                    }
                }
            }
        }

        Self {
            hostname: System::host_name().unwrap_or_else(|| "Unknown".into()),
            os_name: System::name().unwrap_or_else(|| "Windows".into()),
            os_version: System::os_version().unwrap_or_default(),
            uptime_seconds: System::uptime(),
            uuid,
            serial_number: serial,
        }
    }
}
