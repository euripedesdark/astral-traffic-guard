# Astral Traffic Guard — Documentação (Português)

![Licença](https://img.shields.io/badge/License-Proprietária-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![i18n](https://img.shields.io/badge/i18n-4%20Idiomas-green.svg)

Proxy Reverso de Alta Performance, Motor de ACL e WAF em Rust, projetado para ambientes corporativos com requisitos estritos de segurança, controle e auditoria.

---

## Funcionalidades Principais

1. **Proxy Reverso Layer 7 & TLS Termination**:
   - Balanceamento de carga adaptativo (Round Robin, Least Connections, IP Hash).
   - Terminação e renovação automatizada de certificados TLS/mTLS.
2. **Motor de Decisão ACL & Identidade**:
   - Integração direta com Active Directory / LDAPS e Astral Auth Service.
   - Avaliação de regras por usuário, grupo, domínio e categoria em < 2ms.
3. **Feed Manager de Reputação Web**:
   - Sincronização automática com provedores de Threat Intelligence.
   - Atualizações incrementais de listas de bloqueio sem interrupção de tráfego.
4. **Construtor de Políticas Híbridas**:
   - Regras combinadas por domínio, expressões regulares em cabeçalhos, portas, métodos e faixas horárias UTC.
5. **Painel de Controle em 4 Idiomas**:
   - Interface React 18 + PrimeReact com suporte completo a Português (pt-BR), Inglês (en-US), Espanhol (es-ES) e Francês (fr-FR).
6. **Métricas e Auditoria**:
   - Trilha de auditoria de 200 eventos em memória e banco de dados.
   - Exportador Prometheus e dashboards Grafana.

---

## Estrutura do Repositório

- `src/`: Código-fonte em Rust (serviços, controladores, domínio).
- `frontend/`: Aplicação SPA React 18, Vite, PrimeReact, PrimeFlex.
- `docs/`: Guias de arquitetura, integração e observabilidade.

---

## Licença

Copyright (C) Astral Software. Todos os direitos reservados. Licença Proprietária Comercial.
