mod modules;

use serde::Serialize;
use sysinfo::System;
use std::time::Duration;
// Import para converter a struct em string JSON formatada
use serde_json;

// Imports dos seus módulos
use crate::modules::cpu::CpuMetrics;
use crate::modules::gpu::GpuMetrics;
use crate::modules::memory::MemoryMetrics;
use crate::modules::system::SystemMetrics;
use crate::modules::disk::{DiskMetrics, DiskInfo};
use crate::modules::network::{NetworkMetrics, NetworkInterface};

#[derive(Serialize)]
struct AgentPayload {
    system: SystemMetrics,
    cpu: CpuMetrics,
    memory: MemoryMetrics,
    gpus: Vec<GpuMetrics>,
    disks: Vec<DiskInfo>,
    networks: Vec<NetworkInterface>,
}

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    let client = reqwest::Client::new();
    let api_url = "https://api.omninexus.com.br/v1/telemetry";

    println!("🚀 Agente Omninexus em execução...");

    loop {
        // Atualiza os dados do sistema
        sys.refresh_all();

        // Coleta os dados em nossa struct
        let payload = AgentPayload {
            system: SystemMetrics::collect(&sys),
            cpu: CpuMetrics::collect(&sys),
            memory: MemoryMetrics::collect(&sys),
            gpus: GpuMetrics::collect(),
            disks: DiskMetrics::collect(),
            networks: NetworkMetrics::collect(),
        };

        // --- VISUALIZAÇÃO DO JSON ---
        // Isso imprimirá no seu terminal exatamente o que o Bun vai receber
        match serde_json::to_string_pretty(&payload) {
            Ok(json_string) => {
                println!("\n{:=^50}", " PAYLOAD JSON GENERATED ");
                println!("{}", json_string);
                println!("{:=^50}\n", "");
            }
            Err(e) => eprintln!("❌ Erro ao formatar JSON: {}", e),
        }

        // Envio para o Backend
        println!("📡 Enviando dados para {}...", api_url);

        match client.post(api_url)
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    println!("✅ Sucesso! Host: {} | Status: {}", payload.system.hostname, res.status());
                } else {
                    eprintln!("⚠️ O servidor recebeu, mas retornou erro: {}", res.status());
                }
            }
            Err(e) => {
                eprintln!("❌ Erro de conexão: {}. O backend Bun está rodando?", e);
            }
        }

        // Intervalo de 60 segundos
        println!("😴 Aguardando 60 segundos para a próxima coleta...");
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
