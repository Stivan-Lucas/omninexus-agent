# 🕵️‍♂️ Omninexus Agent (Rust)

O **Omninexus Agent** é um serviço leve de monitoramento de baixo nível, desenvolvido em **Rust 2024**, projetado para coletar telemetria em tempo real de máquinas Windows com o mínimo de impacto em CPU e memória.

Ele faz parte do ecossistema [Omninexus](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-backend), enviando dados coletados via WMI e APIs nativas para o nosso backend processar.

## 🚀 Funcionalidades Atuais

- 🖥️ **System**: Hostname, OS, Uptime e versão do Kernel.
- ⚙️ **CPU**: Uso global, contagem de núcleos e frequência.
- 🧠 **Memory**: Memória RAM total, usada e swap.
- 🎮 **GPU**: Detecção via WMI (NVIDIA, AMD, Intel).
- 💾 **Disk**: Listagem de drives, espaço livre/total e sistema de arquivos.
- 🌐 **Network**: Monitoramento de interfaces, bytes recebidos e enviados.

---

## 🛠️ Stack Tecnológica

- **Linguagem:** [Rust 2024](https://www.rust-lang.org/)
- **Runtime:** [Tokio](https://tokio.rs/) (Async I/O)
- **Coleta:** `sysinfo`, `wmi-rs`, `windows-rs`
- **Automação:** Semantic Release + GitHub Actions

---

## 📦 Ciclo de Release & Versionamento

Este projeto utiliza **Semantic Release**. O binário `.exe` é compilado e anexado automaticamente na aba de [Releases](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-agent/releases) a cada nova funcionalidade.

### Como contribuir (Padrão de Commit)

As mensagens de commit ditam a próxima versão do Agente:

| Tipo         | Gatilho de Versão     | Descrição                                          |
| ------------ | --------------------- | -------------------------------------------------- |
| `feat: ...`  | **Minor** (ex: 1.1.0) | Nova métrica ou funcionalidade de coleta.          |
| `fix: ...`   | **Patch** (ex: 1.0.1) | Correção em algum módulo de coleta ou bug de rede. |
| `chore: ...` | **Nenhum**            | Mudança em configs, CI/CD ou dependências.         |
| `feat!: ...` | **Major** (ex: 2.0.0) | Mudança que quebra a integração com o Backend.     |

---

## 📥 Instalação e Execução

### Usuário (Execução)

1. Vá para a aba [Releases](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-agent/releases).
2. Baixe o `agent.exe` da versão mais recente.
3. Certifique-se de que o [Backend](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-backend) está rodando.
4. Execute o binário.

### Desenvolvedor (Compilação)

Se deseja compilar manualmente, você precisará do Rust instalado.

```bash
# Clone o repositório
git clone https://github.com/Stivan-Lucas/omninexus-agent.git

# Instale ferramentas de automação (opcional, para release)
bun install

# Execute em modo desenvolvimento
cargo run

# Gere o binário otimizado
cargo build --release

```

---

## 🔗 Ecossistema Omninexus

- **Backend:** [omninexus-backend](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-backend) (Ingestão Bun/Fastify)
- **Frontend:** [omninexus-frontend](https://www.google.com/search?q=https://github.com/Stivan-Lucas/omninexus-frontend) (Dashboard Next.js)

---

_Monitoramento de alta performance para sistemas Windows._
