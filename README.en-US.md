# Astral Traffic Guard

![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B%20%2F%20OOP-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![Tokio](https://img.shields.io/badge/Runtime-Tokio%20Async-brightgreen.svg)
![Proxy](https://img.shields.io/badge/Reverse%20Proxy-L7%20WAF%20%2F%20ACL-brightgreen.svg)
![React](https://img.shields.io/badge/Frontend-React%2018%20%2B%20Vite-61dafb.svg)

**High-Performance Reverse Proxy, ACL Engine and Web Policy Enforcement in Rust** — Enterprise traffic management module of the Astral ecosystem. Engineered with Object-Oriented Architecture in Rust, Tokio runtime, and Axum framework for Layer 7 load balancing, TLS termination, access policy control, and anti-DDoS mitigation.

This project is licensed under **GNU AGPL v3**. See [`LICENSE.md`](LICENSE.md).

---

## 🎯 Key Features

- **Object-Oriented Architecture in Rust**:
  - Layered isolation: Domain Models (`domain/`), Polymorphic Services with async traits (`service/`), and REST Controllers (`controller/`).
  - Dynamic dispatch with `Arc<dyn ProxyService>` and `Arc<dyn AclEvaluator>`.
- **Sub-Millisecond ACL Evaluation**:
  - Real-time policy checking (< 2ms) with in-memory caching and resilient directory fallback.
  - Granular allow/block lists, domain wildcards, and user/group overrides.
- **`auth_request` Endpoint**:
  - Route `/api/v1/acl/check` tailored for seamless integration with Nginx, Envoy, or ATS reverse proxies.
- **Modern SPA Console**:
  - React 18, TypeScript, and Vite single-page console with full 4-language i18n support.

---

## 🛠️ Technology Stack

| Layer | Technology | Purpose |
|---|---|---|
| **Core Language** | Rust 1.80+ (Edition 2021/2024) | Predictable sub-millisecond response and memory safety |
| **Architecture** | Object-Oriented (OOP) | Domain traits, encapsulation, dynamic dispatch |
| **Web Server** | Axum 0.8 + Tower | High-throughput asynchronous routing |
| **Async Runtime** | Tokio 1.x (Full) | Non-blocking asynchronous I/O execution |
| **Directory Client** | Ldap3 crate | Active Directory and LDAP query client |
| **Frontend UI** | React 18, TypeScript, Vite | Responsive administrative dashboard |

---

## 💖 Support the Project

Astral Traffic Guard is open-source software maintained by an independent engineer:

### International Transfer (Wise)

| | |
|---|---|
| **Beneficiary** | Euripedes Batista de Paiva Junior |
| **Account Type** | Checking |
| **Routing number (Wire & ACH)** | `101019628` |
| **Account number** | `215822927677` |
| **Bank Name** | Wise US Inc, 108 W 13th St, Wilmington, DE, 19801, United States |
| **SWIFT / BIC** | `TRWIUS35XXX` |

### Brazil (PIX)

```text
24adc62c-b073-4587-974d-03fe35f6733f
```

---

## 📄 License

This project is licensed under **GNU Affero General Public License v3.0 (AGPLv3)**. See [`LICENSE.md`](LICENSE.md).

(c) 2026 Astral Traffic Guard — **Created by: Euripedes Batista de Paiva Junior**
