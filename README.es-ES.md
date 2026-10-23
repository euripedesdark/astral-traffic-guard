# Astral Traffic Guard — Documentación (Español)

![Licencia](https://img.shields.io/badge/License-Propietaria-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![i18n](https://img.shields.io/badge/i18n-4%20Idiomas-green.svg)

Proxy Inverso de Alto Rendimiento, Motor de ACL y WAF en Rust, diseñado para entornos corporativos con altos requerimientos de seguridad, control y auditoría.

---

## Características Principales

1. **Proxy Inverso Capa 7 y Terminación TLS**:
   - Balanceo de carga inteligente (Round Robin, Least Connections, IP Hash).
   - Gestión automatizada de certificados TLS/mTLS.
2. **Motor de Decisión ACL e Identidad**:
   - Integración nativa con Active Directory / LDAPS y Astral Auth Service.
   - Evaluación de reglas en menos de 2ms por usuario, grupo y categoría.
3. **Gestor de Fuentes de Reputación Web**:
   - Sincronización continua de indicadores de inteligencia de amenazas.
   - Actualización atómica de listas de bloqueo sin tiempo de inactividad.
4. **Constructor de Políticas Híbridas**:
   - Reglas multicondición por dominio, expresiones regulares en encabezados, puertos y ventanas horarias UTC.
5. **Panel de Control en 4 Idiomas**:
   - Consola React 18 con soporte completo para Español (es-ES), Portugués (pt-BR), Inglés (en-US) y Francés (fr-FR).
6. **Métricas y Auditoría**:
   - Registro de auditoría y métricas Prometheus integradas con Grafana.

---

## Licencia

Copyright (C) Astral Software. Todos los derechos reservados. Licencia Comercial Propietaria.

## 🔐 Política Unificada de Acceso y Autenticación

**Astral Traffic Guard** adopta la política unificada de gobernanza y control de acceso del ecosistema Astral:

1. **OAuth2 / Samba Active Directory / Postgres**:
   - Autenticación federada orquestada mediante `astral-auth-service` (`http://127.0.0.1:8181`). Los operadores de tráfico y seguridad se autentican contra Samba AD para configurar reglas de proxy y WAF.
2. **Mediante `auth-service` (SSO)**:
   - Validación de encabezados `Authorization: Bearer <token>` y cookies de sesión (`sso_token` y `astral_token`) firmados con el secreto compartido (`AUTH_TOKEN_SECRET` en `/etc/auth-service/auth.env`).
3. **Usuario Local Linux (`root` o `sudo`)**:
   - Autenticación de operador en el sistema operativo para cuentas con privilegios `root` o en los grupos `wheel` o `sudo`.
4. **Superusuario PostgreSQL**:
   - Acceso administrativo para cualquier superusuario PostgreSQL con `rolsuper = true` (ej: `postgres`).
