# NOTICE

Astral Traffic Guard
Copyright (C) 2026 Euripedes Batista de Paiva Junior

Este arquivo atende à seção 4(d) da GNU Affero General Public License v3.0:
registra os avisos de copyright de terceiros e a licença de cada componente de
terceiros com o qual este projeto interage.

Este programa é software livre: você pode redistribuí-lo e modificá-lo sob os
termos da AGPLv3, publicada pela Free Software Foundation. Não há garantia
alguma. O texto completo está em [`LICENSE.md`](LICENSE.md).

---

## 📦 Dependências de runtime e build

| Componente | Licença | Onde é usado |
|---|---|---|
| Axum, Tower, Tokio | MIT / Apache-2.0 | `src/` (backend HTTP e runtime assíncrono em Rust) |
| Ldap3 | MIT / Apache-2.0 | `src/` (integração assíncrona com Active Directory / LDAP) |
| Serde, Serde JSON | MIT / Apache-2.0 | Serialização e deserialização de dados |
| Tracing, Tracing Subscriber | MIT | Logs e observabilidade estruturada |
| React, React DOM | MIT | `frontend/src/` |
| Vite, TypeScript | MIT | `frontend/` (build tool e tipagem estática) |
