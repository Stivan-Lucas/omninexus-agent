use serde::Serialize;
use sysinfo::System;

/// Representa as métricas de um núcleo lógico de CPU
#[derive(Serialize)]
pub struct CpuCore {
    /// Identificador do núcleo (índice lógico)
    pub id: usize,

    /// Percentual de uso do núcleo (0.0 a 100.0)
    pub usage: f32,

    /// Frequência atual do núcleo em MHz
    pub frequency: u64,
}

/// Agregado completo de métricas da CPU do sistema
#[derive(Serialize)]
pub struct CpuMetrics {
    /// Modelo do processador (ex: Intel(R) Core(TM)...)
    pub model: String,

    /// Identificador do fabricante (ex: GenuineIntel, AuthenticAMD)
    pub vendor_id: String,

    /// Uso global da CPU em percentual
    pub global_usage: f32,

    /// Quantidade de núcleos físicos (quando disponível)
    pub physical_core_count: Option<usize>,

    /// Quantidade de núcleos lógicos
    pub logical_core_count: usize,

    /// Detalhamento por núcleo lógico
    pub cores: Vec<CpuCore>,
}

impl CpuMetrics {
    /// Coleta métricas da CPU a partir de uma instância já atualizada do `System`
    ///
    /// ⚠️ Importante:
    /// O chamador é responsável por garantir que `sys.refresh_cpu()` ou
    /// `sys.refresh_all()` tenha sido executado previamente.
    pub fn collect(sys: &System) -> Self {
        let cpus = sys.cpus();

        let cores = cpus
            .iter()
            .enumerate()
            .map(|(id, cpu)| CpuCore {
                id,
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect::<Vec<_>>();

        let cpu_info = cpus.first();

        Self {
            model: cpu_info
                .map(|cpu| cpu.brand().to_owned())
                .unwrap_or_default(),

            vendor_id: cpu_info
                .map(|cpu| cpu.vendor_id().to_owned())
                .unwrap_or_default(),

            global_usage: sys.global_cpu_usage(),
            physical_core_count: sys.physical_core_count(),
            logical_core_count: cpus.len(),
            cores,
        }
    }
}
