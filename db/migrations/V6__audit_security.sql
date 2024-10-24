-- ##########################################
-- Audit and Security System
-- ##########################################

-- Security classification levels
CREATE TABLE security_classifications (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    access_level INTEGER NOT NULL, -- Numeric level for hierarchical access
    requires_clearance BOOLEAN DEFAULT false,
    clearance_procedure TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- User security clearances
CREATE TABLE user_security_clearances (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    classification_id BIGINT NOT NULL REFERENCES security_classifications(id),
    granted_by BIGINT NOT NULL REFERENCES users(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    revoked_at TIMESTAMP WITH TIME ZONE,
    revoked_by BIGINT REFERENCES users(id),
    revocation_reason TEXT,
    CONSTRAINT unique_user_clearance UNIQUE (user_id, classification_id)
);

-- Authentication audit
CREATE TABLE authentication_logs (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT REFERENCES users(id),
    event_type TEXT NOT NULL, -- e.g., 'LOGIN', 'LOGOUT', '2FA_ATTEMPT'
    status TEXT NOT NULL, -- e.g., 'SUCCESS', 'FAILURE'
    ip_address INET,
    user_agent TEXT,
    location TEXT,
    device_fingerprint TEXT,
    failure_reason TEXT,
    additional_details JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Activity audit
CREATE TABLE activity_logs (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Actor Information
    user_id BIGINT REFERENCES users(id),
    role_id BIGINT REFERENCES roles(id),

    -- Action Details
    action_type TEXT NOT NULL,
    action_category TEXT NOT NULL,
    action_description TEXT NOT NULL,

    -- Target Entity
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    entity_name TEXT,

    -- Context
    case_id BIGINT,
    case_filed_date DATE,
    court_id BIGINT REFERENCES courts(id),

    -- Request Details
    ip_address INET,
    user_agent TEXT,
    session_id TEXT,

    -- Additional Information
    status TEXT NOT NULL,
    changes_before JSONB,
    changes_after JSONB,
    metadata JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Security incidents
CREATE TABLE security_incidents (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Incident Details
    incident_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL,

    -- Discovery Information
    detected_at TIMESTAMP WITH TIME ZONE NOT NULL,
    detected_by BIGINT REFERENCES users(id),
    reported_at TIMESTAMP WITH TIME ZONE,
    reported_by BIGINT REFERENCES users(id),

    -- Incident Description
    description TEXT NOT NULL,
    affected_systems TEXT[],
    affected_users TEXT[],
    potential_impact TEXT,

    -- Response
    response_steps JSONB,
    mitigation_steps JSONB,
    resolution_notes TEXT,
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolved_by BIGINT REFERENCES users(id),

    -- Investigation
    investigation_status TEXT,
    evidence_collected JSONB,
    forensic_notes TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Access control lists
CREATE TABLE access_control_lists (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    resource_type TEXT NOT NULL,
    resource_id BIGINT NOT NULL,
    principal_type TEXT NOT NULL, -- e.g., 'USER', 'ROLE', 'GROUP'
    principal_id BIGINT NOT NULL,
    permissions JSONB NOT NULL,
    granted_by BIGINT NOT NULL REFERENCES users(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_acl_entry UNIQUE (resource_type, resource_id, principal_type, principal_id)
);

-- Data access policies
CREATE TABLE data_access_policies (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    policy_type TEXT NOT NULL,
    rules JSONB NOT NULL,
    applies_to TEXT[], -- Array of entity types
    priority INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Data retention policies
CREATE TABLE data_retention_policies (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    entity_type TEXT NOT NULL,
    retention_period INTERVAL NOT NULL,
    archive_strategy TEXT NOT NULL,
    deletion_strategy TEXT NOT NULL,
    exceptions_handling JSONB,
    is_active BOOLEAN DEFAULT true,
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_retention_policy UNIQUE (entity_type)
);

-- Encryption keys management
CREATE TABLE encryption_keys (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    key_type TEXT NOT NULL,
    key_identifier TEXT NOT NULL UNIQUE,
    key_version INTEGER NOT NULL,
    status TEXT NOT NULL,
    creation_date TIMESTAMP WITH TIME ZONE NOT NULL,
    activation_date TIMESTAMP WITH TIME ZONE,
    expiration_date TIMESTAMP WITH TIME ZONE,
    created_by BIGINT NOT NULL REFERENCES users(id),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_user_clearances_user ON user_security_clearances(user_id);
CREATE INDEX idx_auth_logs_user ON authentication_logs(user_id);
CREATE INDEX idx_auth_logs_created_at ON authentication_logs(created_at);
CREATE INDEX idx_activity_logs_user ON activity_logs(user_id);
CREATE INDEX idx_activity_logs_entity ON activity_logs(entity_type, entity_id);
CREATE INDEX idx_activity_logs_case ON activity_logs(case_id, case_filed_date);
CREATE INDEX idx_security_incidents_status ON security_incidents(status);
CREATE INDEX idx_acl_resource ON access_control_lists(resource_type, resource_id);
CREATE INDEX idx_acl_principal ON access_control_lists(principal_type, principal_id);
CREATE INDEX idx_data_policies_type ON data_access_policies(policy_type);
CREATE INDEX idx_encryption_keys_identifier ON encryption_keys(key_identifier);

-- Triggers
CREATE OR REPLACE FUNCTION log_security_classification_change()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO activity_logs (
        user_id,
        action_type,
        action_category,
        action_description,
        entity_type,
        entity_id,
        changes_before,
        changes_after
    ) VALUES (
        current_setting('app.current_user_id')::bigint,
        'SECURITY_CLASSIFICATION_CHANGE',
        'SECURITY',
        'Security classification modified',
        'SECURITY_CLASSIFICATION',
        NEW.id::text,
        row_to_json(OLD),
        row_to_json(NEW)
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER security_classification_audit
    AFTER UPDATE ON security_classifications
    FOR EACH ROW
    EXECUTE FUNCTION log_security_classification_change();

-- Add timestamp update triggers
CREATE TRIGGER update_security_classifications_timestamp
    BEFORE UPDATE ON security_classifications
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_security_incidents_timestamp
    BEFORE UPDATE ON security_incidents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_access_control_lists_timestamp
    BEFORE UPDATE ON access_control_lists
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_data_access_policies_timestamp
    BEFORE UPDATE ON data_access_policies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_data_retention_policies_timestamp
    BEFORE UPDATE ON data_retention_policies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_encryption_keys_timestamp
    BEFORE UPDATE ON encryption_keys
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();
