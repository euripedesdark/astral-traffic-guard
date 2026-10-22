-- Apply on database astral as a dedicated migration owner, after 20261022_secure_proxy.sql.
-- JSONB conditions is deliberately selected: load one ordered policy snapshot per request;
-- one-to-many writes are avoided; add a cache with invalidation before high-throughput use.
CREATE TABLE IF NOT EXISTS public.proxy_policies (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL CHECK(length(name) BETWEEN 1 AND 160),
    action text NOT NULL CHECK(action IN ('ALLOW','DENY','INSPECT')),
    conditions jsonb NOT NULL CHECK(jsonb_typeof(conditions) = 'array'),
    priority integer NOT NULL DEFAULT 100,
    enabled boolean NOT NULL DEFAULT true,
    created_by text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_proxy_policies_order ON public.proxy_policies(priority DESC,id ASC) WHERE enabled;
REVOKE ALL ON public.proxy_policies FROM PUBLIC,proxy;
GRANT SELECT,INSERT,DELETE ON public.proxy_policies TO proxy;
ALTER TABLE public.proxy_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY proxy_policy_read ON public.proxy_policies FOR SELECT TO proxy USING (true);
CREATE POLICY proxy_policy_create ON public.proxy_policies FOR INSERT TO proxy WITH CHECK (true);
CREATE POLICY proxy_policy_delete ON public.proxy_policies FOR DELETE TO proxy USING (true);
-- IMPORTANT: keep the database owner outside runtime; authorization is enforced by Axum.
-- Production requires separate read-only enforcement role and separate admin role; see docs.
