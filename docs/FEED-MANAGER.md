# Feed Manager — deployment and security checklist

## Implemented
- `src/domain/proxy.rs`: `ReputationSource`, `WebCategory`, `DomainReputation`.
- `src/service/feed_manager.rs`: HTTPS TXT/CSV (first column) downloader; configured exact-host allowlist; redirects forbidden; 8 MB response cap; 100,000 indicator cap; normalized domain/IP storage; transactional replacement; Moka TTL 120s and 20,000-entry cache.
- `src/main.rs`: Tokio feed scheduler every six hours after startup; only active when auth, database and `ASTRAL_FEED_ALLOWED_HOSTS` are configured.
- `src/service/policy_engine.rs`: clears untrusted incoming `category`; fills it from the reputation resolver before matching `WebCategory` conditions. Database/reputation errors fail closed.
- `frontend/src/modules/proxy/FeedManagerModule.tsx`: add, list, refresh and category assignment.
- `migrations/20261022_reputation_feeds.sql`: categories, feed sources and indicators.

## Settings
`ASTRAL_FEED_ALLOWED_HOSTS`: comma-separated EXACT feed FQDNs allowed for download. URLs must be HTTPS, have no embedded credentials and no custom port. Never allow arbitrary user-supplied feed origins.
`ASTRAL_DATABASE_URL`, `AUTH_SERVICE_INTROSPECT_URL`, `AUTH_SERVICE_CLIENT_TOKEN` required by secure proxy as documented in SECURE-PROXY-ARCHITECTURE.md.
Apply the SQL migration while connected to the `astral` database as a separate migration owner.
Always configure egress network firewall rules and DNS/IP destination restrictions; host allowlisting alone does NOT eliminate DNS rebinding or compromised upstream risks.

## Important limitations before production
- **No network traffic adapter** calls `PolicyEvaluator::evaluate` yet. Feed categories affect service decisions only, not actual egress blocks.
- Legacy ACL defaults to ALLOW and is not replaced.
- Admin frontend still needs compatible Bearer/session bridge; existing fetch helper relies on cookies.
- Feed scheduling is one task per instance: for HA deployments, use a distributed lock or leader election to avoid duplicated jobs.
- HTTP body size is bounded, but large feeds are inserted row by row in a transaction; add COPY/staging tables, batched import, database advisory lock, feed signatures, and source provenance/conflict policies for scale.
- TXT is one indicator per line; CSV parser currently reads **only first field**; headers and complex quoted CSV are not fully supported.
- IP lookup is exact. CIDR blocklists are not supported. Category overlap presently prefers malware then phishing, otherwise uses lexicographic indicator order; define explicit precedence before production.
- The category cache TTL means external database updates may remain stale up to 120s on other nodes; provide distributed invalidation.
- A persisted feed refresh operation is currently logged via `tracing` but has no protected SQL audit record, requiring a later immutable audit integration.
- No malware feed URL defaults are installed or assumed trustworthy. Review list licensing, freshness, false positives, privacy and operator authorization.
- No test or build execution has been performed by this assistant and Cargo.lock may require regeneration. Run `cargo check --all-targets`, `cargo test --all-targets`, and `npm run build`.

## Category policies
Create a policy with `conditions: [{"type":"web_category","value":"gambling"}]` and action `DENY`. A trusted traffic adapter must construct RequestContext from authenticated identity and actual network details. Never trust category supplied by request clients.
