-- ##########################################
-- Case Management System
-- ##########################################

-- Partition function for cases by year
CREATE FUNCTION create_case_partition(year INTEGER)
RETURNS void AS $$
BEGIN
    EXECUTE format(
        'CREATE TABLE IF NOT EXISTS cases_%s ' ||
        'PARTITION OF cases ' ||
        'FOR VALUES FROM (%L) TO (%L)',
        year,
        format('%s-01-01', year),
        format('%s-01-01', year + 1)
    );
END;
$$ LANGUAGE plpgsql;

-- Core cases table (partitioned by filed_date)
CREATE TABLE cases (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Case Identification
    case_number TEXT NOT NULL,
    title TEXT NOT NULL,
    case_type TEXT NOT NULL,
    nature_of_suit TEXT,
    filing_type TEXT NOT NULL,

    -- Status and Dates
    status TEXT NOT NULL,
    filed_date DATE NOT NULL,
    closed_date DATE,
    reopened_date DATE,

    -- Court Information
    court_id BIGINT NOT NULL REFERENCES courts(id),
    division_id BIGINT REFERENCES court_divisions(id),
    assigned_judge_id BIGINT REFERENCES judicial_officers(id),
    magistrate_judge_id BIGINT REFERENCES judicial_officers(id),

    -- Case Details
    jury_demand TEXT,
    demand_amount NUMERIC(15,2),
    jurisdictional_basis TEXT,

    -- Statistical Reporting
    statistical_close_date DATE,
    disposition_method TEXT,
    disposition_date DATE,

    -- Security and Access Control
    security_level TEXT NOT NULL DEFAULT 'PUBLIC',
    sealed BOOLEAN DEFAULT false,
    sealed_date DATE,
    sealed_by BIGINT REFERENCES users(id),

    -- Metadata
    created_by BIGINT NOT NULL REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (id, filed_date)
) PARTITION BY RANGE (filed_date);

-- Create initial partitions
SELECT create_case_partition(generate_series(2020, 2025));

-- Case related cases junction table
CREATE TABLE related_cases (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    related_case_id BIGINT NOT NULL,
    related_case_filed_date DATE NOT NULL,
    relationship_type TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date),
    FOREIGN KEY (related_case_id, related_case_filed_date) REFERENCES cases (id, filed_date),
    CONSTRAINT unique_case_relationship UNIQUE (case_id, related_case_id, relationship_type)
);

-- ##########################################
-- Case Parties and Representation
-- ##########################################

-- Case party types
CREATE TABLE party_types (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Case parties
CREATE TABLE case_parties (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    party_type_id BIGINT NOT NULL REFERENCES party_types(id),
    name TEXT NOT NULL,
    is_lead BOOLEAN DEFAULT false,

    -- Contact Information
    address JSONB,
    phone TEXT,
    email TEXT,

    -- Additional Details
    is_pro_se BOOLEAN DEFAULT false,
    company_details JSONB,
    individual_details JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

-- Attorney representations junction table
CREATE TABLE party_attorney_representations (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_party_id BIGINT NOT NULL REFERENCES case_parties(id),
    attorney_id BIGINT NOT NULL REFERENCES users(id),
    representation_type TEXT NOT NULL, -- e.g., 'LEAD', 'LOCAL', 'OF_COUNSEL'
    representation_start_date DATE NOT NULL,
    representation_end_date DATE,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    notes TEXT,
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_party_attorney_representation
        UNIQUE (case_party_id, attorney_id, representation_start_date)
);

-- ##########################################
-- Case Events and Docket Entries
-- ##########################################

-- Event type enumeration table
CREATE TABLE event_types (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    description TEXT,
    requires_document BOOLEAN DEFAULT false,
    requires_judicial_review BOOLEAN DEFAULT false,
    auto_notification BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Case events/docket entries
CREATE TABLE case_events (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    event_type_id BIGINT NOT NULL REFERENCES event_types(id),

    -- Event Details
    event_date TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    public_entry TEXT, -- Public facing description
    internal_notes TEXT, -- Internal notes visible only to court staff

    -- Document Management
    document_number TEXT,
    page_count INTEGER,

    -- Event Metadata
    filed_by BIGINT REFERENCES users(id),
    entered_by BIGINT NOT NULL REFERENCES users(id),
    event_status TEXT NOT NULL DEFAULT 'ACTIVE',

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

-- Event history tracking
CREATE TABLE event_history (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    event_id BIGINT NOT NULL REFERENCES case_events(id),
    changed_by BIGINT NOT NULL REFERENCES users(id),
    change_type TEXT NOT NULL,
    changes_before JSONB,
    changes_after JSONB,
    change_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ##########################################
-- Case Deadlines and Scheduling
-- ##########################################

-- Case deadlines table
CREATE TABLE case_deadlines (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,

    -- Deadline Details
    title TEXT NOT NULL,
    deadline_type TEXT NOT NULL,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    completion_date TIMESTAMP WITH TIME ZONE,

    -- Additional Information
    description TEXT,
    priority TEXT NOT NULL DEFAULT 'NORMAL',

    -- Status and Tracking
    status TEXT NOT NULL DEFAULT 'PENDING',
    reminder_days INTEGER[], -- Array of days before deadline for reminders

    -- Metadata
    created_by BIGINT NOT NULL REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

-- Deadline assignments junction table
CREATE TABLE deadline_assignments (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    deadline_id BIGINT NOT NULL REFERENCES case_deadlines(id),
    assigned_user_id BIGINT NOT NULL REFERENCES users(id),
    assigned_by BIGINT NOT NULL REFERENCES users(id),
    assigned_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    notes TEXT,
    completed_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_deadline_assignment
        UNIQUE (deadline_id, assigned_user_id)
);

-- Deadline history tracking
CREATE TABLE deadline_history (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    deadline_id BIGINT NOT NULL REFERENCES case_deadlines(id),
    changed_by BIGINT NOT NULL REFERENCES users(id),
    change_type TEXT NOT NULL,
    changes_before JSONB,
    changes_after JSONB,
    change_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ##########################################
-- Case Status History
-- ##########################################

-- Case status history tracking
CREATE TABLE case_status_history (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    old_status TEXT NOT NULL,
    new_status TEXT NOT NULL,
    changed_by BIGINT NOT NULL REFERENCES users(id),
    change_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

-- ##########################################
-- Indexes
-- ##########################################

-- Cases indexes
CREATE UNIQUE INDEX idx_cases_case_number_filed_date ON cases (case_number, filed_date);
CREATE INDEX idx_cases_court_id ON cases (court_id);
CREATE INDEX idx_cases_assigned_judge ON cases (assigned_judge_id);
CREATE INDEX idx_cases_status ON cases (status);
CREATE INDEX idx_cases_created_by ON cases (created_by);

-- Related cases indexes
CREATE INDEX idx_related_cases_case_id ON related_cases (case_id, case_filed_date);
CREATE INDEX idx_related_cases_related_case_id ON related_cases (related_case_id, related_case_filed_date);

-- Case parties indexes
CREATE INDEX idx_case_parties_case_id_date ON case_parties (case_id, case_filed_date);
CREATE INDEX idx_case_parties_party_type ON case_parties (party_type_id);
CREATE INDEX idx_case_parties_name ON case_parties (name);

-- Attorney representations indexes
CREATE INDEX idx_party_attorney_case_party ON party_attorney_representations(case_party_id);
CREATE INDEX idx_party_attorney_attorney ON party_attorney_representations(attorney_id);
CREATE INDEX idx_party_attorney_status ON party_attorney_representations(status);

-- Case events indexes
CREATE INDEX idx_case_events_case_id_date ON case_events (case_id, case_filed_date);
CREATE INDEX idx_case_events_event_type ON case_events (event_type_id);
CREATE INDEX idx_case_events_event_date ON case_events (event_date);
CREATE INDEX idx_case_events_filed_by ON case_events (filed_by);
CREATE INDEX idx_case_events_entered_by ON case_events (entered_by);

-- Case deadlines indexes
CREATE INDEX idx_case_deadlines_case_id_date ON case_deadlines (case_id, case_filed_date);
CREATE INDEX idx_case_deadlines_due_date ON case_deadlines (due_date);
CREATE INDEX idx_case_deadlines_status ON case_deadlines (status);
CREATE INDEX idx_case_deadlines_created_by ON case_deadlines (created_by);

-- Deadline assignments indexes
CREATE INDEX idx_deadline_assignments_deadline ON deadline_assignments(deadline_id);
CREATE INDEX idx_deadline_assignments_user ON deadline_assignments(assigned_user_id);
CREATE INDEX idx_deadline_assignments_assigner ON deadline_assignments(assigned_by);
CREATE INDEX idx_deadline_assignments_status ON deadline_assignments(status);

-- ##########################################
-- Triggers
-- ##########################################

-- Case status change logging
CREATE OR REPLACE FUNCTION log_case_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status <> OLD.status THEN
        INSERT INTO case_status_history (
            case_id,
            case_filed_date,
            old_status,
            new_status,
            changed_by
        ) VALUES (
            NEW.id,
            NEW.filed_date,
            OLD.status,
            NEW.status,
            NEW.updated_by
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Event history tracking
CREATE OR REPLACE FUNCTION log_event_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        INSERT INTO event_history (
            event_id,
            changed_by,
            change_type,
            changes_before,
            changes_after
        ) VALUES (
            NEW.id,
            CURRENT_USER::bigint,
            'UPDATE',
            row_to_json(OLD),
            row_to_json(NEW)
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Deadline history tracking
CREATE OR REPLACE FUNCTION log_deadline_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        INSERT INTO deadline_history (
            deadline_id,
            changed_by,
            change_type,
            changes_before,
            changes_after
        ) VALUES (
            NEW.id,
            COALESCE(NEW.updated_by, CURRENT_USER::bigint),
            'UPDATE',
            row_to_json(OLD),
            row_to_json(NEW)
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply triggers
CREATE TRIGGER case_status_change
    AFTER UPDATE OF status ON cases
    FOR EACH ROW
    EXECUTE FUNCTION log_case_status_change();

CREATE TRIGGER track_event_changes
    AFTER UPDATE ON case_events
    FOR EACH ROW
    EXECUTE FUNCTION log_event_changes();

CREATE TRIGGER track_deadline_changes
    AFTER UPDATE ON case_deadlines
    FOR EACH ROW
    EXECUTE FUNCTION log_deadline_changes();

-- Timestamp update triggers
CREATE TRIGGER update_cases_timestamp
    BEFORE UPDATE ON cases
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_case_parties_timestamp
    BEFORE UPDATE ON case_parties
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_party_attorney_representations_timestamp
    BEFORE UPDATE ON party_attorney_representations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_case_events_timestamp
    BEFORE UPDATE ON case_events
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_case_deadlines_timestamp
    BEFORE UPDATE ON case_deadlines
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_deadline_assignments_timestamp
    BEFORE UPDATE ON deadline_assignments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

-- ##########################################
-- Views
-- ##########################################

-- Case party representations view
CREATE OR REPLACE VIEW vw_party_representations AS
SELECT
    cp.id AS party_id,
    cp.case_id,
    cp.case_filed_date,
    cp.name AS party_name,
    cp.is_pro_se,
    pt.name AS party_type,
    u.username AS attorney_name,
    par.representation_type,
    par.representation_start_date,
    par.representation_end_date,
    par.status AS representation_status
FROM case_parties cp
LEFT JOIN party_types pt ON cp.party_type_id = pt.id
LEFT JOIN party_attorney_representations par ON cp.id = par.case_party_id
LEFT JOIN users u ON par.attorney_id = u.id;

-- Case events detailed view
CREATE OR REPLACE VIEW vw_case_events_detailed AS
SELECT
    ce.id AS event_id,
    ce.case_id,
    c.case_number,
    et.name AS event_type,
    ce.event_date,
    ce.title,
    ce.description,
    ce.public_entry,
    f.username AS filed_by_user,
    e.username AS entered_by_user,
    ce.event_status,
    ce.created_at,
    ce.updated_at
FROM case_events ce
JOIN cases c ON ce.case_id = c.id AND ce.case_filed_date = c.filed_date
JOIN event_types et ON ce.event_type_id = et.id
LEFT JOIN users f ON ce.filed_by = f.id
LEFT JOIN users e ON ce.entered_by = e.id;

-- Case deadlines detailed view
CREATE OR REPLACE VIEW vw_case_deadlines_detailed AS
SELECT
    cd.id AS deadline_id,
    cd.case_id,
    c.case_number,
    cd.title,
    cd.deadline_type,
    cd.due_date,
    cd.status AS deadline_status,
    cd.priority,
    da.assigned_user_id,
    u.username AS assigned_to,
    a.username AS assigned_by,
    da.assigned_date,
    da.status AS assignment_status,
    da.completed_date,
    cd.created_at,
    cd.updated_at
FROM case_deadlines cd
JOIN cases c ON cd.case_id = c.id AND cd.case_filed_date = c.filed_date
LEFT JOIN deadline_assignments da ON cd.id = da.deadline_id
LEFT JOIN users u ON da.assigned_user_id = u.id
LEFT JOIN users a ON da.assigned_by = a.id;
