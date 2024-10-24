-- ##########################################
-- Workflow and Calendar Management System
-- ##########################################

-- Workflow templates
CREATE TABLE workflow_templates (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    steps JSONB NOT NULL,
    transitions JSONB NOT NULL,
    required_roles JSONB,
    sla_config JSONB,
    form_templates JSONB,
    is_active BOOLEAN DEFAULT true,
    deactivated_at TIMESTAMP WITH TIME ZONE,
    deactivated_by BIGINT REFERENCES users(id),
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Active workflows
CREATE TABLE workflows (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    template_id BIGINT NOT NULL REFERENCES workflow_templates(id),
    case_id BIGINT,
    case_filed_date DATE,
    title TEXT NOT NULL,
    description TEXT,
    current_step TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    priority TEXT NOT NULL DEFAULT 'NORMAL',
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    target_completion_date TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    supervisor_id BIGINT REFERENCES users(id),
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Create workflow assignments junction table
CREATE TABLE workflow_assignments (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    workflow_id BIGINT NOT NULL REFERENCES workflows(id),
    assigned_user_id BIGINT NOT NULL REFERENCES users(id),
    assigned_by BIGINT NOT NULL REFERENCES users(id),
    assigned_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    role_type TEXT,  -- e.g., 'OWNER', 'PARTICIPANT', 'REVIEWER'
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_workflow_assignment
        UNIQUE (workflow_id, assigned_user_id)
);


-- Add indexes for the new table
CREATE INDEX idx_workflow_assignments_workflow
    ON workflow_assignments(workflow_id);
CREATE INDEX idx_workflow_assignments_user
    ON workflow_assignments(assigned_user_id);
CREATE INDEX idx_workflow_assignments_status
    ON workflow_assignments(status);

-- Add timestamp trigger
CREATE TRIGGER update_workflow_assignments_timestamp
    BEFORE UPDATE ON workflow_assignments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

-- Create view for easy querying of workflow assignments
CREATE OR REPLACE VIEW vw_workflow_assignments AS
SELECT
    w.id AS workflow_id,
    w.title AS workflow_title,
    w.status AS workflow_status,
    w.current_step,
    u.username AS assigned_to,
    wa.role_type,
    wa.status AS assignment_status,
    wa.assigned_date,
    a.username AS assigned_by
FROM workflows w
JOIN workflow_assignments wa ON w.id = wa.workflow_id
JOIN users u ON wa.assigned_user_id = u.id
JOIN users a ON wa.assigned_by = a.id;
-- Workflow history
CREATE TABLE workflow_history (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    workflow_id BIGINT NOT NULL REFERENCES workflows(id),
    step_from TEXT,
    step_to TEXT,
    action_taken TEXT NOT NULL,
    action_result TEXT NOT NULL,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    comments TEXT,
    attachments JSONB,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Calendar events
CREATE TABLE calendar_events (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title TEXT NOT NULL,
    description TEXT,
    event_type TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'NORMAL',
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    all_day BOOLEAN DEFAULT false,
    recurrence_rule TEXT,
    location_type TEXT NOT NULL,
    location_details JSONB,
    case_id BIGINT,
    case_filed_date DATE,
    workflow_id BIGINT REFERENCES workflows(id),
    organizer_id BIGINT NOT NULL REFERENCES users(id),
    participants JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'SCHEDULED',
    canceled_reason TEXT,
    canceled_by BIGINT REFERENCES users(id),
    canceled_at TIMESTAMP WITH TIME ZONE,
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Calendar event responses
CREATE TABLE calendar_responses (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    event_id BIGINT NOT NULL REFERENCES calendar_events(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    response_status TEXT NOT NULL,
    response_message TEXT,
    responded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_event_response UNIQUE (event_id, user_id)
);

-- Resource availability
CREATE TABLE resource_availability (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    resource_type TEXT NOT NULL,
    resource_id BIGINT NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    status TEXT NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_resource_timespan UNIQUE (resource_type, resource_id, start_time, end_time)
);

-- Scheduling conflicts
CREATE TABLE scheduling_conflicts (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    event_id BIGINT NOT NULL REFERENCES calendar_events(id),
    conflict_type TEXT NOT NULL,
    conflict_description TEXT NOT NULL,
    severity TEXT NOT NULL,
    resolution_status TEXT NOT NULL DEFAULT 'PENDING',
    resolved_by BIGINT REFERENCES users(id),
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolution_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Task management
CREATE TABLE tasks (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title TEXT NOT NULL,
    description TEXT,
    task_type TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'NORMAL',
    workflow_id BIGINT REFERENCES workflows(id),
    case_id BIGINT,
    case_filed_date DATE,
    assigned_to BIGINT REFERENCES users(id),
    assigned_by BIGINT NOT NULL REFERENCES users(id),
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE,
    completed_date TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL DEFAULT 'PENDING',
    completion_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Indexes
CREATE INDEX idx_workflows_template ON workflows(template_id);
CREATE INDEX idx_workflows_case ON workflows(case_id, case_filed_date);
CREATE INDEX idx_workflows_status ON workflows(status);
CREATE INDEX idx_workflow_history_workflow ON workflow_history(workflow_id);
CREATE INDEX idx_calendar_events_case ON calendar_events(case_id, case_filed_date);
CREATE INDEX idx_calendar_events_workflow ON calendar_events(workflow_id);
CREATE INDEX idx_calendar_events_timespan ON calendar_events(start_time, end_time);
CREATE INDEX idx_calendar_responses_event ON calendar_responses(event_id);
CREATE INDEX idx_resource_availability_timespan ON resource_availability(resource_type, resource_id, start_time, end_time);
CREATE INDEX idx_scheduling_conflicts_event ON scheduling_conflicts(event_id);
CREATE INDEX idx_tasks_workflow ON tasks(workflow_id);
CREATE INDEX idx_tasks_case ON tasks(case_id, case_filed_date);
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX idx_tasks_status ON tasks(status);

-- Triggers for conflict detection
CREATE OR REPLACE FUNCTION check_calendar_conflicts()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM calendar_events
        WHERE id != NEW.id
        AND case_id = NEW.case_id
        AND case_filed_date = NEW.case_filed_date
        AND (
            (start_time, end_time) OVERLAPS (NEW.start_time, NEW.end_time)
        )
    ) THEN
        INSERT INTO scheduling_conflicts (
            event_id,
            conflict_type,
            conflict_description,
            severity
        ) VALUES (
            NEW.id,
            'TIME_OVERLAP',
            'Event overlaps with existing case events',
            'HIGH'
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_calendar_event_conflicts
    AFTER INSERT OR UPDATE ON calendar_events
    FOR EACH ROW
    EXECUTE FUNCTION check_calendar_conflicts();

-- Timestamp update triggers
CREATE TRIGGER update_workflow_templates_timestamp
    BEFORE UPDATE ON workflow_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_workflows_timestamp
    BEFORE UPDATE ON workflows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_calendar_events_timestamp
    BEFORE UPDATE ON calendar_events
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_resource_availability_timestamp
    BEFORE UPDATE ON resource_availability
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_scheduling_conflicts_timestamp
    BEFORE UPDATE ON scheduling_conflicts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_tasks_timestamp
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();
