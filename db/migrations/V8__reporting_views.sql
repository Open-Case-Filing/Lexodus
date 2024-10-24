-- ##########################################
-- Reporting and Analytics Views
-- ##########################################

-- Case Status Overview
CREATE OR REPLACE VIEW vw_case_status_overview AS
SELECT
    c.court_id,
    co.name as court_name,
    c.status,
    EXTRACT(YEAR FROM c.filed_date::timestamp) as filing_year,
    COUNT(*) as case_count,
    AVG(EXTRACT(EPOCH FROM (COALESCE(c.closed_date::timestamp, CURRENT_TIMESTAMP) - c.filed_date::timestamp))/86400)::integer as avg_days_to_resolution,
    COUNT(CASE WHEN c.status = 'OPEN' THEN 1 END) as open_cases,
    COUNT(CASE WHEN c.status = 'CLOSED' THEN 1 END) as closed_cases
FROM cases c
JOIN courts co ON c.court_id = co.id
GROUP BY c.court_id, co.name, c.status, filing_year;

-- Judicial Workload Analysis
CREATE OR REPLACE VIEW vw_judicial_workload AS
SELECT
    jo.id as judge_id,
    u.full_name as judge_name,
    COUNT(DISTINCT c.id) as total_cases,
    COUNT(DISTINCT CASE WHEN c.status = 'OPEN' THEN c.id END) as active_cases,
    COUNT(DISTINCT ce.id) as total_hearings,
    COUNT(DISTINCT d.id) as total_decisions
FROM judicial_officers jo
JOIN users u ON jo.user_id = u.id
LEFT JOIN cases c ON c.assigned_judge_id = jo.id
LEFT JOIN calendar_events ce ON ce.case_id = c.id AND ce.event_type = 'HEARING'
LEFT JOIN documents d ON d.case_id = c.id
LEFT JOIN document_types dt ON d.document_type_id = dt.id AND dt.category = 'DECISION'
GROUP BY jo.id, u.full_name;

-- Case Timeline Analysis
CREATE OR REPLACE VIEW vw_case_timeline_metrics AS
SELECT
    c.id as case_id,
    c.case_number,
    c.filed_date,
    c.status,
    COUNT(DISTINCT ce.id) as total_events,
    COUNT(DISTINCT d.id) as total_documents,
    COUNT(DISTINCT p.id) as total_parties,
    MIN(ce.start_time) as first_event_date,
    MAX(ce.start_time) as last_event_date,
    MAX(ce.start_time) FILTER (WHERE ce.event_type = 'HEARING') as last_hearing_date,
    COUNT(DISTINCT m.id) as total_motions,
    COUNT(DISTINCT CASE WHEN m.status = 'PENDING' THEN m.id END) as pending_motions
FROM cases c
LEFT JOIN calendar_events ce ON c.id = ce.case_id
LEFT JOIN documents d ON c.id = d.case_id
LEFT JOIN case_parties p ON c.id = p.case_id
LEFT JOIN motions m ON c.id = m.case_id
GROUP BY c.id, c.case_number, c.filed_date, c.status;

-- Document Analytics
CREATE OR REPLACE VIEW vw_document_analytics AS
SELECT
    dt.category as document_category,
    dt.name as document_type,
    COUNT(d.id) as total_documents,
    AVG(d.page_count) as avg_page_count,
    SUM(d.file_size)/1024/1024 as total_size_mb,
    COUNT(DISTINCT d.case_id) as cases_affected,
    COUNT(DISTINCT d.filed_by) as unique_filers
FROM documents d
JOIN document_types dt ON d.document_type_id = dt.id
GROUP BY dt.category, dt.name;

-- Workflow Efficiency Metrics
CREATE OR REPLACE VIEW vw_workflow_efficiency AS
SELECT
    wt.name as workflow_template,
    wt.category,
    COUNT(w.id) as total_instances,
    AVG(EXTRACT(EPOCH FROM (w.completed_at - w.start_date))/86400)::integer as avg_completion_days,
    COUNT(CASE WHEN w.status = 'COMPLETED' THEN 1 END) as completed_workflows,
    COUNT(CASE WHEN w.status = 'ACTIVE' THEN 1 END) as active_workflows,
    COUNT(DISTINCT w.created_by) as unique_initiators
FROM workflow_templates wt
LEFT JOIN workflows w ON wt.id = w.template_id
GROUP BY wt.id, wt.name, wt.category;

-- User Activity Analysis
CREATE OR REPLACE VIEW vw_user_activity_analysis AS
SELECT
    u.username,
    r.name as role_name,
    COUNT(al.id) as total_actions,
    COUNT(DISTINCT al.action_type) as unique_action_types,
    COUNT(DISTINCT al.entity_id) as entities_affected,
    MAX(al.created_at) as last_activity,
    COUNT(DISTINCT al.case_id) as cases_involved
FROM users u
JOIN roles r ON u.role_id = r.id
LEFT JOIN activity_logs al ON u.id = al.user_id
WHERE al.created_at >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY u.id, u.username, r.name;

-- Court Performance Metrics
CREATE OR REPLACE VIEW vw_court_performance_metrics AS
WITH case_metrics AS (
    SELECT
        c.court_id,
        COUNT(*) as total_cases,
        AVG(EXTRACT(EPOCH FROM (COALESCE(c.closed_date::timestamp, CURRENT_TIMESTAMP) - c.filed_date::timestamp))/86400) as avg_resolution_time,
        COUNT(DISTINCT c.assigned_judge_id) as active_judges
    FROM cases c
    WHERE c.filed_date >= CURRENT_DATE - INTERVAL '1 year'
    GROUP BY c.court_id
)
SELECT
    co.name as court_name,
    co.district,
    cm.total_cases,
    cm.avg_resolution_time::integer as avg_days_to_resolution,
    cm.active_judges,
    COUNT(DISTINCT ce.id) as total_hearings,
    COUNT(DISTINCT d.id) as total_filings
FROM courts co
LEFT JOIN case_metrics cm ON co.id = cm.court_id
LEFT JOIN cases c ON c.court_id = co.id
LEFT JOIN calendar_events ce ON ce.case_id = c.id AND ce.event_type = 'HEARING'
    AND ce.created_at >= CURRENT_DATE - INTERVAL '1 year'
LEFT JOIN documents d ON d.case_id = c.id
    AND d.created_at >= CURRENT_DATE - INTERVAL '1 year'
GROUP BY co.id, co.name, co.district, cm.total_cases, cm.avg_resolution_time, cm.active_judges;

-- Security and Audit Analytics
CREATE OR REPLACE VIEW vw_security_audit_analytics AS
SELECT
    DATE_TRUNC('day', al.created_at) as audit_date,
    al.action_category,
    COUNT(*) as total_actions,
    COUNT(DISTINCT al.user_id) as unique_users,
    COUNT(DISTINCT al.entity_id) as unique_entities,
    COUNT(CASE WHEN al.status = 'FAILURE' THEN 1 END) as failed_actions,
    COUNT(DISTINCT al.case_id) as cases_affected
FROM activity_logs al
WHERE al.created_at >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY DATE_TRUNC('day', al.created_at), al.action_category
ORDER BY audit_date DESC;

-- Create materialized view for performance
CREATE MATERIALIZED VIEW mv_case_statistics AS
SELECT
    c.court_id,
    c.status,
    c.case_type,
    DATE_TRUNC('month', c.filed_date::timestamp) as filing_month,
    COUNT(*) as case_count,
    AVG(EXTRACT(EPOCH FROM (COALESCE(c.closed_date::timestamp, CURRENT_TIMESTAMP) - c.filed_date::timestamp))/86400)::integer as avg_days_open,
    COUNT(DISTINCT c.assigned_judge_id) as judges_involved,
    COUNT(DISTINCT d.id) as document_count,
    COUNT(DISTINCT ce.id) as event_count
FROM cases c
LEFT JOIN documents d ON c.id = d.case_id
LEFT JOIN calendar_events ce ON c.id = ce.case_id
GROUP BY c.court_id, c.status, c.case_type, DATE_TRUNC('month', c.filed_date::timestamp);

-- Create index for materialized view
CREATE UNIQUE INDEX idx_mv_case_statistics ON mv_case_statistics (court_id, status, case_type, filing_month);

-- Create refresh function for materialized view
CREATE OR REPLACE FUNCTION refresh_case_statistics()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY mv_case_statistics;
END;
$$ LANGUAGE plpgsql;

-- Create automated refresh trigger function
CREATE OR REPLACE FUNCTION trigger_refresh_case_statistics()
RETURNS trigger AS $$
BEGIN
    PERFORM refresh_case_statistics();
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to refresh materialized view
CREATE TRIGGER refresh_case_statistics_trigger
AFTER INSERT OR UPDATE OR DELETE
ON cases
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_case_statistics();

-- Add indexes for view performance
CREATE INDEX IF NOT EXISTS idx_activity_logs_created_at ON activity_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_activity_logs_action_category ON activity_logs(action_category);
CREATE INDEX IF NOT EXISTS idx_calendar_events_start_time ON calendar_events(start_time);
CREATE INDEX IF NOT EXISTS idx_cases_filing_year ON cases(EXTRACT(YEAR FROM filed_date));
CREATE INDEX IF NOT EXISTS idx_calendar_events_event_type ON calendar_events(event_type);
