# Astral Traffic Guard — Documentation (Français)

![Licence](https://img.shields.io/badge/License-Propriétaire-red.svg)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)
![i18n](https://img.shields.io/badge/i18n-4%20Langues-green.svg)

Proxy Inverse Haute Performance, Moteur d'ACL et WAF en Rust, conçu pour les infrastructures d'entreprise exigeant sécurité maximale, contrôle de trafic et auditabilité.

---

## Fonctionnalités Principales

1. **Proxy Inverse Couche 7 & Terminaison TLS**:
   - Équilibrage de charge adaptatif (Round Robin, Least Connections, IP Hash).
   - Terminaison et gestion automatisée des certificats TLS/mTLS.
2. **Moteur d'ACL et Résolution d'Identité**:
   - Intégration directe avec Active Directory / LDAPS et Astral Auth Service.
   - Évaluation des politiques en moins de 2ms par identité, groupe et catégorie.
3. **Gestionnaire de Flux de Réputation Web**:
   - Ingestion automatique de flux de cybermenaces (malware, phishing, botnets).
   - Mises à jour atomiques en mémoire sans interruption de service.
4. **Générateur de Politiques Hybrides**:
   - Règles combinées multicritères (domaines, regex d'en-têtes, ports, méthodes HTTP, plages horaires UTC).
5. **Interface SPA en 4 Langues**:
   - Tableau de bord React 18 avec bascule dynamique entre Français (fr-FR), Portugais (pt-BR), Anglais (en-US) et Espagnol (es-ES).
6. **Observabilité et Traçabilité**:
   - Journalisation d'audit en temps réel et métriques Prometheus pour Grafana.

---

## Licence

Copyright (C) Astral Software. Tous droits réservés. Licence Commerciale Propriétaire.

## 🔐 Politique d'Accès Unifié et Authentification

**Astral Traffic Guard** adopte la politique unifiée de gouvernance et de contrôle d'accès de l'écosystème Astral :

1. **OAuth2 / Samba Active Directory / Postgres** :
   - Authentification fédérée orchestrée via `astral-auth-service` (`http://127.0.0.1:8181`). Les opérateurs de trafic et de sécurité s'authentifient auprès de Samba AD pour configurer le proxy et le WAF.
2. **Via `auth-service` (SSO)** :
   - Validation des en-têtes `Authorization: Bearer <token>` et des cookies de session (`sso_token` et `astral_token`) signés avec le secret partagé (`AUTH_TOKEN_SECRET` dans `/etc/auth-service/auth.env`).
3. **Utilisateur Local Linux (`root` ou `sudo`)** :
   - Authentification système d'opérateur pour les comptes disposant des privilèges `root` ou membres des groupes `wheel` ou `sudo`.
4. **Superutilisateur PostgreSQL** :
   - Accès administratif pour tout compte PostgreSQL avec `rolsuper = true` (ex. `postgres`).
