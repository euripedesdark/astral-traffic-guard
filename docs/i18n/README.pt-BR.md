# Astral Traffic Guard

![Licença](https://img.shields.io/badge/License-AGPLv3-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B%20%2F%20OOP-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![Tokio](https://img.shields.io/badge/Runtime-Tokio%20Async-brightgreen.svg)
![Proxy](https://img.shields.io/badge/Reverse%20Proxy-L7%20WAF%20%2F%20ACL-brightgreen.svg)
![React](https://img.shields.io/badge/Frontend-React%2018%20%2B%20Vite-61dafb.svg)

**Proxy Reverso de Alta Performance, Motor de ACL e Filtro Web em Rust** — Módulo especializado do ecossistema Astral responsável pelo controle de navegação, inspeção de tráfego, terminação TLS e aplicação de políticas de acesso corporativas.

Este projeto está sob **GNU AGPL v3**. Ver [`LICENSE.md`](LICENSE.md).

---

## 🎯 Principais Recursos

- **Arquitetura Orientada a Objetos em Rust**:
  - Modelagem forte em camadas: Entidades de Domínio (`domain/`), Serviços com Traits (`service/`), e Controladores REST (`controller/`).
  - Polimorfismo e dynamic dispatch (`Arc<dyn ProxyService>`, `Arc<dyn AclEvaluator>`).
- **Motor de ACL de Alta Performance**:
  - Avaliação de regras em tempo real (< 2ms) com cache em memória e fallback resiliente.
  - Políticas granulares de bloqueio/liberação com regras de exceção e overrides.
- **Endpoint `auth_request`**:
  - Rota `/api/v1/acl/check` desenhada para integração direta com proxies reversos, validando tokens Bearer JWT ou sessões ativas.
- **Painel de Controle SPA**:
  - Interface moderna em React 18 + TypeScript + Vite para criação e edição de categorias, listas de domínios e regras de acesso por perfil, com i18n em 4 idiomas.

---

## 🏗️ Estrutura do Projeto

```
astral-traffic-guard/
├── Cargo.toml                                # Manifest Rust (Axum, Tokio, Ldap3, Serde, Tower)
├── Cargo.lock                                # Lockfile de dependências Rust
├── src/
│   ├── main.rs                               # Ponto de entrada da aplicação
│   ├── server.rs                             # Configuração do servidor HTTP Axum
│   ├── domain/                               # Modelos de domínio e regras de ACL
│   ├── service/                              # Camada de serviços (traits e lógica de proxy/ACL)
│   ├── controller/                           # Controladores de rotas REST
│   └── config/                               # Carregamento de configurações
├── frontend/                                 # Painel SPA (React + TypeScript + Vite)
├── scripts/                                  # Scripts de integração e observabilidade
├── docs/                                     # Documentação técnica e guias i18n
├── LICENSE.md                                # Licença GNU AGPLv3
└── DONATE.md                                 # Dados de apoio e doações
```

---

## 🛠️ Stack Tecnológica

| Camada | Tecnologia | Finalidade |
|---|---|---|
| **Linguagem Principal** | Rust 1.80+ (Edition 2021/2024) | Alta velocidade, latência determinística e segurança de memória |
| **Design Pattern** | Orientado a Objetos (OOP) | Domain models, traits, injeção de dependência e dynamic dispatch |
| **Framework Web** | Axum 0.8 + Tower | Roteamento assíncrono de tráfego |
| **Runtime Assíncrono** | Tokio 1.x (Full) | I/O assíncrono não bloqueante |
| **Autenticação de Diretório** | Ldap3 crate | Consulta a Active Directory e LDAP corporativo |
| **Frontend** | React 18, TypeScript, Vite | Interface web administrativa |

---

## 🚀 Como Executar

```bash
# Compilar o backend em Rust
cargo build --release

# Executar o serviço
./target/release/astral-traffic-guard
```

---

## 💖 Apoie o Projeto

Astral Traffic Guard é mantido por um único desenvolvedor independente:

**PIX:**
```text
24adc62c-b073-4587-974d-03fe35f6733f
```

### Transferência Internacional (Wise)

| | |
|---|---|
| **Nome** | Euripedes Batista de Paiva Junior |
| **Tipo de conta** | Checking |
| **Routing number (wire e ACH)** | `101019628` |
| **Número da conta** | `215822927677` |
| **Banco** | Wise US Inc, 108 W 13th St, Wilmington, DE, 19801, United States |
| **SWIFT/BIC** | `TRWIUS35XXX` |

---

## 📄 Licença

Este projeto está sob **GNU AGPL v3**. Ver [`LICENSE.md`](LICENSE.md).

(c) 2026 Astral Traffic Guard — **Criado por: Euripedes Batista de Paiva Junior**
