-- Core Performance Indexes
CREATE INDEX IF NOT EXISTS idx_cases_status_date
ON cases (status, filed_date);

CREATE INDEX IF NOT EXISTS idx_cases_court_date
ON cases (court_id, filed_date);

CREATE INDEX IF NOT EXISTS idx_cases_judge
ON cases (assigned_judge_id, filed_date);

CREATE INDEX IF NOT EXISTS idx_case_events_case
ON case_events (case_id, case_filed_date);

CREATE INDEX IF NOT EXISTS idx_case_events_date
ON case_events (event_date);

CREATE INDEX IF NOT EXISTS idx_documents_case
ON documents (case_id, case_filed_date);

CREATE INDEX IF NOT EXISTS idx_documents_type
ON documents (document_type_id);

-- Partial Index for Active Cases
CREATE INDEX IF NOT EXISTS idx_active_cases
ON cases (filed_date)
WHERE status = 'OPEN';
-- Remove incorrect document index
DROP INDEX IF EXISTS idx_documents_search;

-- Create corrected document indexes
CREATE INDEX IF NOT EXISTS idx_documents_search
ON documents (case_id, document_type_id, filed_date)
INCLUDE (title);

CREATE INDEX IF NOT EXISTS idx_documents_type_status
ON documents (document_type_id, created_at);

CREATE INDEX IF NOT EXISTS idx_documents_case_lookup
ON documents (case_id, created_at, document_type_id);
-- ##########################################
-- Performance and Scalability Optimizations
-- ##########################################

-- 1. Create Optimized Indexes
CREATE INDEX IF NOT EXISTS idx_cases_common_lookup
ON cases (status, filed_date, court_id)
INCLUDE (case_number, title);

CREATE INDEX IF NOT EXISTS idx_documents_search
ON documents (case_id, document_type_id, filed_date)
INCLUDE (title, file_path);

-- Create partial indexes for common conditions
CREATE INDEX IF NOT EXISTS idx_active_cases
ON cases (filed_date, court_id)
WHERE status = 'OPEN';

CREATE INDEX IF NOT EXISTS idx_pending_tasks
ON tasks (due_date, assigned_to)
WHERE status = 'PENDING';

-- 2. Create maintenance function
CREATE OR REPLACE FUNCTION perform_table_maintenance()
RETURNS void AS $$
BEGIN
    -- Analyze tables for better query planning
    ANALYZE cases;
    ANALYZE documents;
    ANALYZE case_events;
    ANALYZE activity_logs;

    -- Vacuum full analysis
    VACUUM (FULL, ANALYZE) cases;
    VACUUM (FULL, ANALYZE) documents;
    VACUUM (FULL, ANALYZE) case_events;
    VACUUM (FULL, ANALYZE) activity_logs;
END;
$$ LANGUAGE plpgsql;

-- 3. Create Database Statistics Management
CREATE TABLE database_statistics (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    collected_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    table_name TEXT NOT NULL,
    total_rows BIGINT,
    total_size BIGINT,
    index_size BIGINT,
    vacuum_count BIGINT,
    dead_tuples BIGINT
);

-- Create statistics collection function
CREATE OR REPLACE FUNCTION collect_database_statistics()
RETURNS void AS $$
BEGIN
    INSERT INTO database_statistics (
        table_name,
        total_rows,
        total_size,
        index_size,
        vacuum_count,
        dead_tuples
    )
    SELECT
        schemaname || '.' || tablename,
        n_live_tup,
        pg_total_relation_size(schemaname || '.' || tablename),
        pg_indexes_size(schemaname || '.' || tablename),
        n_dead_tup,
        vacuum_count
    FROM pg_stat_user_tables
    WHERE schemaname = 'public';
END;
$$ LANGUAGE plpgsql;

-- 4. Create Maintenance Schedule
CREATE TABLE maintenance_schedule (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    maintenance_type TEXT NOT NULL,
    last_run TIMESTAMP WITH TIME ZONE,
    next_run TIMESTAMP WITH TIME ZONE,
    status TEXT,
    duration INTERVAL,
    error_log TEXT
);

-- Initialize maintenance schedule
INSERT INTO maintenance_schedule
(maintenance_type, next_run) VALUES
('VACUUM_ANALYZE', CURRENT_TIMESTAMP + INTERVAL '1 day'),
('COLLECT_STATISTICS', CURRENT_TIMESTAMP + INTERVAL '6 hours');

-- Create maintenance scheduler function
CREATE OR REPLACE FUNCTION process_maintenance_schedule()
RETURNS void AS $$
DECLARE
    task RECORD;
BEGIN
    FOR task IN SELECT * FROM maintenance_schedule WHERE next_run <= CURRENT_TIMESTAMP
    LOOP
        BEGIN
            CASE task.maintenance_type
                WHEN 'VACUUM_ANALYZE' THEN
                    PERFORM perform_table_maintenance();
                WHEN 'COLLECT_STATISTICS' THEN
                    PERFORM collect_database_statistics();
            END CASE;

            UPDATE maintenance_schedule
            SET last_run = CURRENT_TIMESTAMP,
                next_run = CASE maintenance_type
                    WHEN 'VACUUM_ANALYZE' THEN CURRENT_TIMESTAMP + INTERVAL '1 day'
                    WHEN 'COLLECT_STATISTICS' THEN CURRENT_TIMESTAMP + INTERVAL '6 hours'
                END,
                status = 'SUCCESS',
                duration = CURRENT_TIMESTAMP - last_run
            WHERE id = task.id;

        EXCEPTION WHEN OTHERS THEN
            UPDATE maintenance_schedule
            SET status = 'ERROR',
                error_log = SQLERRM
            WHERE id = task.id;
        END;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- 5. Create additional performance indexes
CREATE INDEX IF NOT EXISTS idx_activity_logs_created_at
ON activity_logs(created_at);

CREATE INDEX IF NOT EXISTS idx_activity_logs_user_id
ON activity_logs(user_id);

CREATE INDEX IF NOT EXISTS idx_documents_created_at
ON documents(created_at);

CREATE INDEX IF NOT EXISTS idx_case_events_created_at
ON case_events(created_at);

-- 6. Add computed columns for common calculations
ALTER TABLE cases
ADD COLUMN IF NOT EXISTS days_open INTEGER
GENERATED ALWAYS AS (
    EXTRACT(DAY FROM (COALESCE(closed_date::timestamp, CURRENT_TIMESTAMP) - filed_date::timestamp))
) STORED;

-- Create index on computed column
CREATE INDEX IF NOT EXISTS idx_cases_days_open
ON cases(days_open);
