# Astral Traffic Guard

![Licença](https://img.shields.io/badge/License-Proprietary-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B%20%2F%20OOP-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![Tokio](https://img.shields.io/badge/Runtime-Tokio%20Async-brightgreen.svg)
![Proxy](https://img.shields.io/badge/Reverse%20Proxy-L7%20WAF%20%2F%20ACL-brightgreen.svg)
![React](https://img.shields.io/badge/Frontend-React%2018%20%2B%20Vite-61dafb.svg)
![i18n](https://img.shields.io/badge/i18n-4%20Languages-green.svg)

🇧🇷 **Proxy Reverso de Alta Performance, Motor de ACL e WAF em Rust** — Módulo corporativo do ecossistema Astral responsável pelo controle de tráfego, terminação TLS, balanceamento de carga Layer 7, inspeção de políticas web e proteção contra sobrecarga, implementado com arquitetura assíncrona em Rust.

🇺🇸 **High-Performance Reverse Proxy, ACL Engine and WAF in Rust** — Enterprise module of the Astral ecosystem providing intelligent traffic control, TLS termination, Layer 7 load balancing, web policy evaluation, and overload protection, built with asynchronous Rust.

---

## 🌐 Idiomas / Languages

| | Idioma / Language | Documentação / Documentation |
|---|---|---|
| 🇧🇷 | **Português (Brasil)** | [README.pt-BR.md](README.pt-BR.md) |
| 🇺🇸 | **English** | [README.en-US.md](README.en-US.md) |
| 🇪🇸 | **Español** | [README.es-ES.md](README.es-ES.md) |
| 🇫🇷 | **Français** | [README.fr-FR.md](README.fr-FR.md) |

---

## 🎯 Principais Recursos / Core Highlights

- **Arquitetura Assíncrona de Ultra-Baixa Latência**:
  - Servidor construído com Rust Tokio + Axum para máximo throughput e uso eficiente de CPU/RAM.
  - Sub-milissegundos (< 2ms) na avaliação de ACLs e regras de roteamento.
- **Motor de ACL e Políticas Híbridas**:
  - Avaliação por domínio, método HTTP, range de IP, portas, cabeçalhos regex, janela de horário e grupos do Active Directory.
  - Endpoint `auth_request` padrão (`/api/v1/acl/check`) para integração de proxies upstream.
- **Feed Manager & Reputação Web**:
  - Importação e sincronização automática de listas de inteligência de ameaças (malware, phishing, botnets).
- **Interface SPA com Suporte a 4 Idiomas**:
  - Console React 18 + Vite com alternância em tempo real entre Português, Inglês, Espanhol e Francês.
- **Observabilidade Corporativa**:
  - Métricas Prometheus e dashboards Grafana pré-integrados.

---

## 🔒 Licença / License

Copyright (C) Astral Software. Todos os direitos reservados. Licença Proprietária Comercial.

## 🔐 Política Unificada de Acesso e Autenticação

O **Astral Traffic Guard** adota a política unificada de governança e controle de acesso do ecossistema Astral:

1. **OAuth2 / Samba Active Directory / Postgres**:
   - Autenticação federada orquestrada via `astral-auth-service` (`http://127.0.0.1:8181`). Operadores e analistas de tráfego/segurança são autenticados contra o Samba AD para aplicação de regras de proxy e WAF.
2. **Via `auth-service` (SSO)**:
   - Validação de cabeçalhos `Authorization: Bearer <token>` e cookies de sessão (`sso_token` e `astral_token`) assinados com o segredo compartilhado (`AUTH_TOKEN_SECRET` em `/etc/auth-service/auth.env`).
3. **Usuário Local Linux (`root` ou `sudo`)**:
   - Autenticação de operador no sistema operacional para contas com privilégios `root` ou nos grupos `wheel` ou `sudo`.
4. **Superusuário PostgreSQL**:
   - Acesso administrativo para qualquer superusuário PostgreSQL com `rolsuper = true` (ex: `postgres`).
