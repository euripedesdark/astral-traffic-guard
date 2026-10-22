-- Apply to the astral database as migration owner after the secure proxy migrations.
CREATE TABLE IF NOT EXISTS web_categories (
 slug text PRIMARY KEY CHECK (slug ~ '^[a-z][a-z0-9_-]{1,63}$'),
 title text NOT NULL
);
INSERT INTO web_categories(slug,title) VALUES
 ('malware','Malware'),('phishing','Phishing'),('adult','Adult Content'),('gambling','Gambling'),('social_media','Social Media'),('news','News'),('custom_blocklist','Custom Blocklist')
 ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS reputation_feeds(
 id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
 name text NOT NULL,
 url text NOT NULL,
 category text NOT NULL REFERENCES web_categories(slug),
 enabled boolean NOT NULL DEFAULT true,
 last_updated timestamptz,
 last_count integer NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS domain_reputation(
 indicator text NOT NULL,
 kind text NOT NULL CHECK(kind IN ('ip','domain')),
 category text NOT NULL REFERENCES web_categories(slug),
 source text NOT NULL CHECK(source IN ('manual','external_feed','ai_analysis')),
 feed_id uuid REFERENCES reputation_feeds(id) ON DELETE CASCADE,
 PRIMARY KEY(indicator,category,source,feed_id)
);
CREATE INDEX IF NOT EXISTS reputation_indicator_lookup ON domain_reputation(indicator);
REVOKE ALL ON web_categories,reputation_feeds,domain_reputation FROM PUBLIC,proxy;
GRANT SELECT ON web_categories TO proxy;
GRANT SELECT,INSERT,UPDATE ON reputation_feeds TO proxy;
GRANT SELECT,INSERT,DELETE ON domain_reputation TO proxy;
-- Runtime role should not own tables. Feed updates must use a separate admin capability.
