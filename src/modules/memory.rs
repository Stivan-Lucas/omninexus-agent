use serde::Serialize;
use sysinfo::System;

/// Snapshot consolidado de métricas de memória do sistema
///
/// Todos os valores são expressos em **kilobytes (KB)**,
/// conforme padrão da crate `sysinfo`.
#[derive(Serialize)]
pub struct MemoryMetrics {
    // =========================
    // Memória Física (RAM)
    // =========================

    /// Quantidade total de RAM instalada no sistema
    pub total_ram_kb: u64,

    /// Quantidade de RAM atualmente em uso
    pub used_ram_kb: u64,

    /// RAM efetivamente disponível para novas alocações
    ///
    /// Este é o valor mais relevante para decisões de capacidade,
    /// pois já desconta caches e buffers reutilizáveis.
    pub available_ram_kb: u64,

    /// Percentual de uso da RAM (0.0 a 100.0)
    pub ram_usage_percent: f32,

    // =========================
    // Memória Virtual (Swap)
    // =========================

    /// Quantidade total de swap configurada no sistema
    pub total_swap_kb: u64,

    /// Quantidade de swap atualmente em uso
    pub used_swap_kb: u64,

    /// Percentual de uso da swap (0.0 a 100.0)
    ///
    /// Uso elevado de swap é um forte indicador de pressão
    /// de memória e possível degradação de performance.
    pub swap_usage_percent: f32,
}

impl MemoryMetrics {
    /// Coleta métricas de memória a partir de uma instância já atualizada do `System`
    ///
    /// ⚠️ Pré-requisito:
    /// O chamador deve garantir que `sys.refresh_memory()` ou
    /// `sys.refresh_all()` tenha sido executado previamente.
    ///
    /// Este método **não executa refresh**, apenas consolida dados.
    pub fn collect(sys: &System) -> Self {
        // =========================
        // Captura de valores brutos
        // =========================

        let total_ram = sys.total_memory();
        let used_ram = sys.used_memory();
        let available_ram = sys.available_memory();

        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        // =========================
        // Cálculo de percentuais
        // =========================

        let ram_usage_percent = if total_ram > 0 {
            (used_ram as f32 / total_ram as f32) * 100.0
        } else {
            0.0
        };

        let swap_usage_percent = if total_swap > 0 {
            (used_swap as f32 / total_swap as f32) * 100.0
        } else {
            0.0
        };

        // =========================
        // Construção do snapshot
        // =========================

        Self {
            total_ram_kb: total_ram,
            used_ram_kb: used_ram,
            available_ram_kb: available_ram,
            ram_usage_percent,

            total_swap_kb: total_swap,
            used_swap_kb: used_swap,
            swap_usage_percent,
        }
    }
}
