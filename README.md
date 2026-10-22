# Astral Traffic Guard

![Licença](https://img.shields.io/badge/License-AGPLv3-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B%20%2F%20OOP-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![Tokio](https://img.shields.io/badge/Runtime-Tokio%20Async-brightgreen.svg)
![Proxy](https://img.shields.io/badge/Reverse%20Proxy-L7%20WAF%20%2F%20ACL-brightgreen.svg)
![React](https://img.shields.io/badge/Frontend-React%2018%20%2B%20Vite-61dafb.svg)

🇧🇷 **Proxy Reverso de Alta Performance, Motor de ACL e WAF em Rust** — Módulo corporativo do ecossistema Astral responsável pelo controle de tráfego, terminação TLS, balanceamento de carga Layer 7, inspeção de políticas web e proteção contra sobrecarga, implementado com arquitetura Orientada a Objetos em Rust assíncrono.

🇺🇸 **High-Performance Reverse Proxy, ACL Engine and WAF in Rust** — Enterprise module of the Astral ecosystem providing intelligent traffic control, TLS termination, Layer 7 load balancing, web policy evaluation, and overload protection, built with Object-Oriented Architecture in asynchronous Rust.

---

## 🌐 Escolha o seu idioma / Choose your language

| | Idioma / Language | Documentação / Documentation |
|---|---|---|
| 🇧🇷 | **Português (Brasil)** | [README.pt-BR.md](README.pt-BR.md) |
| 🇺🇸 | **English** | [README.en-US.md](README.en-US.md) |
| 🇪🇸 | **Español** | [README.es-ES.md](README.es-ES.md) |
| 🇫🇷 | **Français** | [README.fr-FR.md](README.fr-FR.md) |

🇧🇷 [Português](README.pt-BR.md) · 🇺🇸 [English](README.en-US.md) · 🇪🇸 [Español](README.es-ES.md) · 🇫🇷 [Français](README.fr-FR.md)

---

## 🎯 Principais Recursos / Core Highlights

- **Arquitetura Orientada a Objetos em Rust**:
  - Modelagem desacoplada em domínio (`domain/`), camada de serviços polimórficos (`service/`) e controladores (`controller/`) com injeção de dependências e dynamic dispatch via traits assíncronas.
  - Runtime Tokio com servidor Axum de ultra-baixa latência.
- **Motor de ACL e Autorização de Tráfego**:
  - Avaliação de regras de acesso em sub-milissegundos (< 2ms) por usuário, IP ou grupo de diretório.
  - Endpoint `auth_request` padrão (`/api/v1/acl/check`) para integração com Nginx, Envoy ou proxies de borda.
  - Integração nativa com Active Directory / LDAP para resolução de identidade.
- **Balanceamento de Carga e Proxy Reverso**:
  - Balanceamento de carga Layer 7 com algoritmos Round Robin, Least Connections e IP Hash.
  - Proxy transparente com suporte a WebSockets e HTTP/2.
- **Mitigação de Sobrecarga e Anti-DDoS**:
  - Rate Limiting adaptativo por token bucket e leaky bucket.
  - Filtro Web e WAF contra injeções e acessos abusivos.
- **Frontend SPA Moderno**:
  - Painel de administração em React 18, Vite e TypeScript com i18n nativo para 4 idiomas.

---

## 💖 Apoie o Projeto / Support the Project

Astral Traffic Guard é um projeto Open Source mantido por um único desenvolvedor independente.

Se este projeto ajudou você, sua infraestrutura ou sua empresa, considere apoiar o desenvolvimento:

### Brasil (PIX)

```text
24adc62c-b073-4587-974d-03fe35f6733f
```

### Internacional (Wise)

| Campo | Dados |
|---|---|
| **Beneficiário / Name** | Euripedes Batista de Paiva Junior |
| **Tipo de conta / Account Type** | Checking |
| **Routing Number (Wire & ACH)** | `101019628` |
| **Número da conta / Account Number** | `215822927677` |
| **Banco / Bank Name** | Wise US Inc, 108 W 13th St, Wilmington, DE, 19801, United States |
| **SWIFT / BIC** | `TRWIUS35XXX` |

Instruções completas de apoio: [DONATE.md](DONATE.md).

---

## 📄 Licença / License

Distribuído sob a licença **GNU Affero General Public License v3.0 (AGPLv3)**. Consulte [LICENSE.md](LICENSE.md).

(c) 2026 Astral Traffic Guard — **Criado por: Euripedes Batista de Paiva Junior**
