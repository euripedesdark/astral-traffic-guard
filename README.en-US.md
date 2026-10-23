# Astral Traffic Guard — Documentation (English)

![License](https://img.shields.io/badge/License-Proprietary-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![i18n](https://img.shields.io/badge/i18n-4%20Languages-green.svg)

High-Performance Reverse Proxy, ACL Engine, and WAF implemented in asynchronous Rust, designed for enterprise networks requiring high-throughput traffic control and strict auditability.

---

## Core Features

1. **Layer 7 Reverse Proxy & TLS Termination**:
   - Adaptive load balancing (Round Robin, Least Connections, IP Hash).
   - Automated TLS/mTLS termination and certificate lifecycle management.
2. **ACL Decision Engine & Directory Identity**:
   - Native integration with Active Directory / LDAPS and Astral Auth Service.
   - Sub-millisecond (< 2ms) policy evaluation based on user identity, group, and category.
3. **Web Reputation Feed Manager**:
   - Automated ingestion of threat intelligence indicators (malware, phishing, botnets).
   - Non-disruptive, atomic updates to runtime blocklists.
4. **Hybrid Policy Builder**:
   - Composable multi-condition rules: domain patterns, header regexes, ports, HTTP methods, and UTC time windows.
5. **Modern 4-Language Management SPA**:
   - React 18 + PrimeReact dashboard with seamless runtime switching between English (en-US), Portuguese (pt-BR), Spanish (es-ES), and French (fr-FR).
6. **Metrics & Audit Trail**:
   - Audit event logging with 200-entry real-time buffer and database persistence.
   - Native Prometheus metrics and Grafana visualization.

---

## License

Copyright (C) Astral Software. All rights reserved. Commercial Proprietary License.

## 🔐 Unified Access Policy and Authentication

**Astral Traffic Guard** adopts the unified governance and access control policy of the Astral ecosystem:

1. **OAuth2 / Samba Active Directory / Postgres**:
   - Federated authentication orchestrated via `astral-auth-service` (`http://127.0.0.1:8181`). Traffic/security operators are authenticated against Samba AD to configure proxy and WAF rules.
2. **Via `auth-service` (SSO)**:
   - Validation of `Authorization: Bearer <token>` headers and session cookies (`sso_token` and `astral_token`) signed with the shared cluster secret (`AUTH_TOKEN_SECRET` in `/etc/auth-service/auth.env`).
3. **Linux Local User (`root` or `sudo`)**:
   - Host system operator authentication for accounts with `root` privileges or membership in the `wheel` or `sudo` groups.
4. **PostgreSQL Superuser**:
   - Administrative login for any local PostgreSQL role with `rolsuper = true` (e.g., `postgres`).
