-- ##########################################
-- API Management System
-- ##########################################

CREATE TABLE api_keys (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT REFERENCES users(id),
    key_hash TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    permissions JSONB,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    expires_at TIMESTAMP WITH TIME ZONE,
    last_used_at TIMESTAMP WITH TIME ZONE,
    issued_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT REFERENCES users(id),
    revoked_at TIMESTAMP WITH TIME ZONE,
    revoked_by BIGINT REFERENCES users(id),
    revocation_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE api_requests (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    api_key_id BIGINT REFERENCES api_keys(id),
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    request_body JSONB,
    response_status INTEGER,
    response_body JSONB,
    ip_address INET,
    user_agent TEXT,
    processing_time INTEGER,
    error_message TEXT,
    request_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE api_rate_limits (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    api_key_id BIGINT REFERENCES api_keys(id),
    window_size INTERVAL NOT NULL,
    max_requests INTEGER NOT NULL,
    current_count INTEGER DEFAULT 0,
    window_start TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_api_keys_user ON api_keys(user_id);
CREATE INDEX idx_api_keys_status ON api_keys(status);
CREATE INDEX idx_api_requests_key ON api_requests(api_key_id);
CREATE INDEX idx_api_requests_timestamp ON api_requests(request_timestamp);
CREATE INDEX idx_api_rate_limits_key ON api_rate_limits(api_key_id);

-- Triggers
CREATE TRIGGER update_api_keys_timestamp
    BEFORE UPDATE ON api_keys
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_api_rate_limits_timestamp
    BEFORE UPDATE ON api_rate_limits
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

-- Rate limit update function
CREATE OR REPLACE FUNCTION update_rate_limit()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.window_start + NEW.window_size < CURRENT_TIMESTAMP THEN
        NEW.current_count := 1;
        NEW.window_start := CURRENT_TIMESTAMP;
    ELSE
        NEW.current_count := NEW.current_count + 1;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_rate_limit_counter
    BEFORE UPDATE ON api_rate_limits
    FOR EACH ROW
    EXECUTE FUNCTION update_rate_limit();

-- Views (Fixed version with no nested aggregates)
CREATE OR REPLACE VIEW vw_api_key_usage AS
WITH endpoint_stats AS (
    SELECT
        api_key_id,
        endpoint,
        COUNT(*) as endpoint_count
    FROM api_requests
    GROUP BY api_key_id, endpoint
)
SELECT
    ak.id as api_key_id,
    ak.name as key_name,
    u.username,
    COUNT(ar.id) as total_requests,
    COUNT(CASE WHEN ar.response_status >= 400 THEN 1 END) as error_count,
    AVG(ar.processing_time)::INTEGER as avg_processing_time,
    MAX(ar.request_timestamp) as last_request,
    (
        SELECT jsonb_object_agg(endpoint, endpoint_count)
        FROM endpoint_stats
        WHERE api_key_id = ak.id
    ) as endpoint_usage
FROM api_keys ak
JOIN users u ON ak.user_id = u.id
LEFT JOIN api_requests ar ON ar.api_key_id = ak.id
WHERE ak.status = 'ACTIVE'
GROUP BY ak.id, ak.name, u.username;
