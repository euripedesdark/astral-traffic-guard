-- Execute connected to the astral database, with a dedicated migration owner.
-- PostgreSQL role 'proxy' is a DB capability role, NOT an end-user credential.
DO $$ BEGIN
 IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname='proxy') THEN
    CREATE ROLE proxy NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION;
 END IF;
END $$;
GRANT CONNECT ON DATABASE astral TO proxy;
CREATE TABLE IF NOT EXISTS public.proxy_rules (
 id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
 name text NOT NULL CHECK(length(name) BETWEEN 1 AND 160),
 identity_source text NOT NULL CHECK(identity_source IN ('ad','local','any')),
 identity_group text,
 host_pattern text NOT NULL,
 port integer CHECK(port BETWEEN 1 AND 65535),
 action text NOT NULL CHECK(action IN ('ALLOW','DENY')),
 priority integer NOT NULL DEFAULT 100,
 enabled boolean NOT NULL DEFAULT true,
 created_by text NOT NULL,
 created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_proxy_rules_priority ON public.proxy_rules(priority DESC) WHERE enabled;
CREATE TABLE IF NOT EXISTS public.proxy_audit_logs (
 id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
 recorded_at timestamptz NOT NULL DEFAULT now(),
 actor text NOT NULL,
 identity_source text NOT NULL,
 operation text NOT NULL,
 target text NOT NULL,
 outcome text NOT NULL CHECK(outcome IN ('ALLOW','DENY','ERROR')),
 details jsonb NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX IF NOT EXISTS idx_proxy_audit_time ON public.proxy_audit_logs(recorded_at DESC);
-- INSERT-only for runtime DB role. Revoke all changes and ownership from runtime.
REVOKE ALL ON TABLE public.proxy_rules,public.proxy_audit_logs FROM PUBLIC,proxy;
GRANT USAGE ON SCHEMA public TO proxy;
GRANT SELECT,INSERT,DELETE ON public.proxy_rules TO proxy;
GRANT SELECT,INSERT ON public.proxy_audit_logs TO proxy;
-- RLS prevents write operations other than INSERT on audit (even if ordinary grants change).
ALTER TABLE public.proxy_audit_logs ENABLE ROW LEVEL SECURITY;
CREATE POLICY proxy_audit_append_only ON public.proxy_audit_logs FOR INSERT TO proxy WITH CHECK (true);
CREATE POLICY proxy_audit_read ON public.proxy_audit_logs FOR SELECT TO proxy USING (true);
-- Strong immutability requires separate PostgreSQL owner, write-once external log shipping,
-- protected backups and database administrators outside the attacker's trust boundary.
COMMENT ON TABLE public.proxy_audit_logs IS 'Append-only to proxy runtime role; not immutable against owner/superuser';
