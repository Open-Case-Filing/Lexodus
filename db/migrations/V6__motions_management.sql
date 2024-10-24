-- ##########################################
-- Motions Management System
-- ##########################################

-- Motion types configuration
CREATE TABLE motion_types (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL, -- e.g., 'PROCEDURAL', 'DISPOSITIVE', 'DISCOVERY'
    description TEXT,
    requires_hearing BOOLEAN DEFAULT false,
    requires_response BOOLEAN DEFAULT true,
    standard_response_days INTEGER,
    auto_scheduling_enabled BOOLEAN DEFAULT false,
    template_document_id BIGINT REFERENCES documents(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Main motions table
CREATE TABLE motions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    motion_type_id BIGINT NOT NULL REFERENCES motion_types(id),

    -- Motion Details
    title TEXT NOT NULL,
    document_id BIGINT REFERENCES documents(id),
    filed_by BIGINT NOT NULL REFERENCES users(id),
    filed_date TIMESTAMP WITH TIME ZONE NOT NULL,

    -- Status Information
    status TEXT NOT NULL DEFAULT 'PENDING', -- 'PENDING', 'GRANTED', 'DENIED', 'WITHDRAWN'
    decided_by BIGINT REFERENCES judicial_officers(id),
    decision_date TIMESTAMP WITH TIME ZONE,
    decision_document_id BIGINT REFERENCES documents(id),

    -- Dates and Deadlines
    response_deadline DATE,
    hearing_date TIMESTAMP WITH TIME ZONE,

    -- Additional Information
    expedited BOOLEAN DEFAULT false,
    under_seal BOOLEAN DEFAULT false,
    relief_requested TEXT,
    grounds TEXT,
    notes TEXT,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Motion responses tracking
CREATE TABLE motion_responses (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    motion_id BIGINT NOT NULL REFERENCES motions(id),
    document_id BIGINT REFERENCES documents(id),
    response_type TEXT NOT NULL, -- 'OPPOSITION', 'SUPPORT', 'REPLY'
    filed_by BIGINT NOT NULL REFERENCES users(id),
    filed_date TIMESTAMP WITH TIME ZONE NOT NULL,
    response_text TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Motion history logging
CREATE TABLE motion_history (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    motion_id BIGINT NOT NULL REFERENCES motions(id),
    action_type TEXT NOT NULL,
    action_by BIGINT NOT NULL REFERENCES users(id),
    action_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    old_status TEXT,
    new_status TEXT,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Motion hearing requests
CREATE TABLE motion_hearing_requests (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    motion_id BIGINT NOT NULL REFERENCES motions(id),
    requested_by BIGINT NOT NULL REFERENCES users(id),
    requested_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    preferred_date_1 DATE,
    preferred_date_2 DATE,
    preferred_date_3 DATE,
    duration_minutes INTEGER,
    special_requirements TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    scheduled_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Performance indexes
CREATE INDEX idx_motions_case_id_date ON motions(case_id, case_filed_date);
CREATE INDEX idx_motions_type ON motions(motion_type_id);
CREATE INDEX idx_motions_status ON motions(status);
CREATE INDEX idx_motions_filed_by ON motions(filed_by);
CREATE INDEX idx_motions_decided_by ON motions(decided_by);
CREATE INDEX idx_motion_responses_motion ON motion_responses(motion_id);
CREATE INDEX idx_motion_history_motion ON motion_history(motion_id);
CREATE INDEX idx_motion_hearing_requests_motion ON motion_hearing_requests(motion_id);

-- Status change tracking trigger
CREATE OR REPLACE FUNCTION log_motion_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status <> OLD.status THEN
        INSERT INTO motion_history (
            motion_id,
            action_type,
            action_by,
            old_status,
            new_status
        ) VALUES (
            NEW.id,
            'STATUS_CHANGE',
            COALESCE(NEW.decided_by, CURRENT_USER::bigint),
            OLD.status,
            NEW.status
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Auto-schedule hearing function
CREATE OR REPLACE FUNCTION auto_schedule_motion_hearing()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'PENDING' AND
       (SELECT requires_hearing FROM motion_types WHERE id = NEW.motion_type_id) THEN
        INSERT INTO motion_hearing_requests (
            motion_id,
            requested_by,
            requested_date,
            duration_minutes,
            status
        ) VALUES (
            NEW.id,
            NEW.filed_by,
            NEW.filed_date,
            60, -- Default duration
            'PENDING'
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply triggers
CREATE TRIGGER motion_status_change
    AFTER UPDATE OF status ON motions
    FOR EACH ROW
    EXECUTE FUNCTION log_motion_status_change();

CREATE TRIGGER motion_auto_schedule_hearing
    AFTER INSERT ON motions
    FOR EACH ROW
    EXECUTE FUNCTION auto_schedule_motion_hearing();

CREATE TRIGGER update_motions_timestamp
    BEFORE UPDATE ON motions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_motion_responses_timestamp
    BEFORE UPDATE ON motion_responses
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_motion_hearing_requests_timestamp
    BEFORE UPDATE ON motion_hearing_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

-- Helper views
CREATE OR REPLACE VIEW vw_motion_details AS
SELECT
    m.id AS motion_id,
    c.case_number,
    mt.name AS motion_type,
    m.title,
    m.status,
    m.filed_date,
    uf.username AS filed_by,
    ud.username AS decided_by,
    m.decision_date,
    m.response_deadline,
    m.hearing_date,
    COUNT(mr.id) AS response_count
FROM motions m
JOIN cases c ON m.case_id = c.id AND m.case_filed_date = c.filed_date
JOIN motion_types mt ON m.motion_type_id = mt.id
JOIN users uf ON m.filed_by = uf.id
LEFT JOIN judicial_officers jo ON m.decided_by = jo.id
LEFT JOIN users ud ON jo.user_id = ud.id
LEFT JOIN motion_responses mr ON m.id = mr.motion_id
GROUP BY
    m.id, c.case_number, mt.name, m.title, m.status,
    m.filed_date, uf.username, ud.username, m.decision_date,
    m.response_deadline, m.hearing_date;

-- Statistical analysis view
CREATE OR REPLACE VIEW vw_motion_statistics AS
SELECT
    mt.name AS motion_type,
    mt.category,
    COUNT(*) as total_motions,
    COUNT(CASE WHEN m.status = 'PENDING' THEN 1 END) as pending_motions,
    COUNT(CASE WHEN m.status = 'GRANTED' THEN 1 END) as granted_motions,
    COUNT(CASE WHEN m.status = 'DENIED' THEN 1 END) as denied_motions,
    AVG(EXTRACT(EPOCH FROM (m.decision_date - m.filed_date))/86400)::integer as avg_days_to_decision,
    COUNT(CASE WHEN m.expedited THEN 1 END) as expedited_motions,
    COUNT(DISTINCT m.case_id) as cases_affected
FROM motions m
JOIN motion_types mt ON m.motion_type_id = mt.id
GROUP BY mt.id, mt.name, mt.category;

-- Insert common motion types
INSERT INTO motion_types
    (name, category, description, requires_hearing, standard_response_days)
VALUES
    ('Motion to Dismiss', 'DISPOSITIVE', 'Request to dismiss the case', true, 21),
    ('Motion for Summary Judgment', 'DISPOSITIVE', 'Request for judgment based on undisputed facts', true, 21),
    ('Motion to Compel', 'DISCOVERY', 'Request to compel discovery responses', false, 14),
    ('Motion for Extension of Time', 'PROCEDURAL', 'Request for deadline extension', false, 7),
    ('Motion to Strike', 'PROCEDURAL', 'Request to strike pleadings or evidence', false, 14),
    ('Motion for Protective Order', 'DISCOVERY', 'Request for protection from discovery', false, 14),
    ('Motion in Limine', 'TRIAL', 'Request to exclude evidence at trial', true, 14),
    ('Motion for Reconsideration', 'PROCEDURAL', 'Request to reconsider prior ruling', false, 14),
    ('Motion to Seal', 'PROCEDURAL', 'Request to seal documents', false, 7),
    ('Motion for Default Judgment', 'DISPOSITIVE', 'Request for judgment by default', false, 14);
