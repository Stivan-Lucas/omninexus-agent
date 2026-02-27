use serde::Serialize;
use sysinfo::{Networks}; // Note que o NetworkExt sumiu, não é mais necessário

#[derive(Serialize, Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub received_kbps: u64,
    pub transmitted_kbps: u64,
    pub mac_address: String,
    pub ip_v4: String,
}

pub struct NetworkMetrics;

impl NetworkMetrics {
    pub fn collect() -> Vec<NetworkInterface> {
        // Na v0.33, Networks::new_with_refreshed_list() é o caminho correto
        let networks = Networks::new_with_refreshed_list();
        let mut interfaces = Vec::new();

        for (name, data) in &networks {
            // Buscamos o primeiro IPv4 disponível na lista de redes da interface
            // Mudança: .addr agora é um campo, não um método. Removemos o ()
            let ip = data.ip_networks().iter()
                .find(|net| net.addr.is_ipv4())
                .map(|net| net.addr.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            interfaces.push(NetworkInterface {
                name: name.clone(),
                // Cálculo de kbps (Bytes * 8 para bits / 1024)
                received_kbps: (data.received() * 8) / 1024,
                transmitted_kbps: (data.transmitted() * 8) / 1024,
                mac_address: data.mac_address().to_string(),
                ip_v4: ip,
            });
        }

        interfaces
    }
}
