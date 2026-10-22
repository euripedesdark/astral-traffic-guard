# Secure Proxy — Architecture and security review

## Scope of this branch
Rust/Axum service for identity-based *policy evaluation*, PostgreSQL-backed rule management and append-only-to-runtime-role audit records. This branch DOES NOT implement an HTTP forward proxy, packet DPI, TLS interception, DLP or anomaly-driven Auto-Ban. Do not advertise those features as delivered. The existing legacy /auth proxy handler and ACL routes are still separate and must not be treated as enforcing the new decision endpoint.

## Trust boundaries
1. An operator/client presents a Bearer access token to secure-proxy routes.
2. Proxy forwards the token over HTTPS to a configured `AUTH_SERVICE_INTROSPECT_URL`, authenticated using `AUTH_SERVICE_CLIENT_TOKEN`. The token validation/revocation contract MUST be verified against the actual auth-service implementation before enabling.
3. Provider AD is issued **only** by auth-service. POSTGRESQL/local identities can be admitted only if the trusted auth-service returns role proxy (or proxy_admin). A PostgreSQL DB role is **not** an end-user password database; do not send database credentials to clients.
4. `proxy_admin` can manage policy; `proxy_admin` and `proxy_auditor` can read audit; any validated identity can request a decision. All unspecified domains/ports **DENY**.
5. No forwarding component consumes decisions yet. Decisions alone do not enforce perimeter security.

## Environment and deployment
- `ASTRAL_DATABASE_URL`: PostgreSQL DSN for a runtime LOGIN role granted membership in the NOLOGIN role `proxy`. Use secret management, not committed values.
- `AUTH_SERVICE_INTROSPECT_URL`: actual HTTPS endpoint, still requiring contract alignment with auth-service.
- `AUTH_SERVICE_CLIENT_TOKEN`: service-to-service token; rotate periodically.
- Apply `migrations/20261022_secure_proxy.sql` while connected to DB `astral` using separate migration owner; do NOT grant table ownership or superuser to runtime. Migration is plain SQL, not a Flyway dependency.
- Expose management endpoints behind trusted TLS ingress; no wildcard CORS in production; require dedicated access controls on legacy /api/v1/acl routes.
- Ensure management requests deliver an Authorization Bearer header. The current frontend API helper relies on same-origin cookies, so ProxyModule is a standalone component pending a secure authenticated BFF/session integration and route hookup.

## Known security gaps / blockers before production
- The existing LDAP authproxy path binds directly to AD. This branch removes its TLS-verification bypass but does **not** fully migrate that path to auth-service. Disable the old handler in deployments requiring centralized-only authentication.
- Existing ACL code defaults to ALLOW and has permissive domain substring matches; it is NOT the new secure policy engine.
- Current PostgreSQL audit guarantees insert-only privileges for its runtime role, not cryptographic immutability against database owners/superusers; ship audit to externally protected append-only storage/SIEM with retention and integrity verification.
- No built-in credential-hash login implementation. Local user authentication belongs to the centralized auth-service and must issue provider/identityId claims for provider POSTGRESQL.
- The existing UI authentication and the new Bearer requirement need bridging before ProxyModule can operate.
- DPI, DLP, CA integration/TLS inspection, traffic forwarding, identity-bound egress enforcement, category isolation, Auto-Ban/Panic integration, telemetry export and integration tests remain pending.
- Run `cargo check --all-targets`, `cargo test --all-targets`, `npm run build`, migration dry-run, and deny-by-default regression tests in CI. These were not executed by the assistant.

## Control mapping
ISO/IEC 27001:2022 Annex A mappings (2013 numbering from original request in parentheses):
| Capability | 2022 | Legacy 2013 |
|---|---|---|
| identity checks / admin authorization | 5.15, 5.16, 8.2, 8.3 | A.9 |
| TLS on introspection, managed keys / CA work pending | 8.24 | A.10 |
| audit and monitoring / external immutability pending | 8.15, 8.16 | A.12.4 |
| deny-default policy, segmentation & inspection pending | 8.20, 8.21, 8.22, 8.23 | A.13 |
| alert/incident workflow integration pending | 5.24–5.28 | A.16 |
These mappings document design intent, **not** certification, guaranteed compliance, or a completed control implementation.

## Hybrid policy engine extension
- `src/domain/policy.rs`: typed `PolicyCondition`, `ProxyPolicy`, `RequestContext`, `PolicyDecision`.
- `src/service/policy_engine.rs`: trait `PolicyEvaluator` with `Arc<dyn PolicyEvaluator>`; PostgreSQL ordered by `priority DESC, id ASC`; AND on conditions; first match wins; default deny.
- `src/controller/policy_controller.rs`: protected CRUD. No public policy evaluation endpoint is exposed because client-supplied source IP, zone, group or URL would be spoofable.
- `migrations/20261022_hybrid_proxy_policies.sql`: JSONB for bounded serialized conditions (24 max), indexed policy ordering; this is an evaluation-read-friendly tradeoff, not full 1:N relational normalization. Rules must be validated by the Rust domain/service.
- `frontend/src/modules/proxy/ProxyPolicyModule.tsx`: dynamic policy-condition builder and priority table; currently requires authenticated Bearer token support in shared frontend client, not yet connected.
- `INSPECT` intentionally fails closed (DENY) until a working TLS inspection and content-inspection path exists. Do not equate `PolicyDecision::Deny` with active network enforcement without a traffic-adapter.
- Categories, application and zone fields are **verified inputs** to be populated by trusted gateway/inspection adapters; do not accept values from arbitrary clients.
- Regex checks enforce pattern length and compiled-memory limits, but a per-request regex compilation and full database read requires cache/benchmark tuning.
- The legacy `/auth` handler and `/api/v1/acl/check` are not yet replaced, so their enforcement behavior is unchanged. The legacy ACL defaults to ALLOW.
- CORS wildcard has been removed from the API router; use a deliberate allowlist in deployment if cross-origin access is required.
- CA issuance, TLS termination, active DPI, DLP content scan, URL-category data feed, incident engine linkage, verified identity/IP attribution and secure log delivery are still missing.
- No Rust build or browser tests have run in the tool environment; lockfile dependency versions must be updated before merge.
