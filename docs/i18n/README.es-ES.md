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
