-- Court System and Judicial Structure
CREATE TABLE courts (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL,
    court_type TEXT NOT NULL,
    district TEXT NOT NULL,
    circuit TEXT NOT NULL,
    jurisdiction TEXT[],
    physical_address TEXT,
    mailing_address TEXT,
    phone TEXT,
    email TEXT,
    website TEXT,
    filing_hours JSONB,
    emergency_contact TEXT,
    is_active BOOLEAN DEFAULT true,
    parent_court_id BIGINT REFERENCES courts(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_court_name_district UNIQUE (name, district)
);

CREATE TABLE judicial_officers (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    court_id BIGINT NOT NULL REFERENCES courts(id),
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    appointment_date DATE NOT NULL,
    term_start_date DATE,
    term_end_date DATE,
    specialties TEXT[],
    chambers_info JSONB,
    is_chief_judge BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_court UNIQUE (user_id, court_id)
);

CREATE TABLE court_divisions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    court_id BIGINT NOT NULL REFERENCES courts(id),
    name TEXT NOT NULL,
    division_type TEXT NOT NULL,
    presiding_judge_id BIGINT REFERENCES judicial_officers(id),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_division_court UNIQUE (court_id, name)
);

CREATE TABLE court_staff (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    court_id BIGINT NOT NULL REFERENCES courts(id),
    division_id BIGINT REFERENCES court_divisions(id),
    role_type TEXT NOT NULL,
    supervisor_id BIGINT REFERENCES court_staff(id),
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_staff_assignment UNIQUE (user_id, court_id, division_id)
);

CREATE TABLE court_calendar_configs (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    court_id BIGINT NOT NULL REFERENCES courts(id),
    division_id BIGINT REFERENCES court_divisions(id),
    calendar_type TEXT NOT NULL,
    day_of_week INTEGER,
    start_time TIME,
    end_time TIME,
    location TEXT,
    max_cases INTEGER,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_courts_parent_court_id ON courts(parent_court_id);
CREATE INDEX idx_judicial_officers_court_id ON judicial_officers(court_id);
CREATE INDEX idx_judicial_officers_user_id ON judicial_officers(user_id);
CREATE INDEX idx_court_divisions_court_id ON court_divisions(court_id);
CREATE INDEX idx_court_divisions_presiding_judge ON court_divisions(presiding_judge_id);
CREATE INDEX idx_court_staff_court_id ON court_staff(court_id);
CREATE INDEX idx_court_staff_division_id ON court_staff(division_id);
CREATE INDEX idx_court_staff_user_id ON court_staff(user_id);
CREATE INDEX idx_calendar_configs_court_id ON court_calendar_configs(court_id);
CREATE INDEX idx_calendar_configs_division_id ON court_calendar_configs(division_id);

CREATE TRIGGER update_courts_timestamp
    BEFORE UPDATE ON courts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_judicial_officers_timestamp
    BEFORE UPDATE ON judicial_officers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_court_divisions_timestamp
    BEFORE UPDATE ON court_divisions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_court_staff_timestamp
    BEFORE UPDATE ON court_staff
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_calendar_configs_timestamp
    BEFORE UPDATE ON court_calendar_configs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();
