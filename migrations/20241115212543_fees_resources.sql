-- 1. First recreate tables in correct order
-- DROP TABLE IF EXISTS financial_transactions CASCADE;
-- DROP TABLE IF EXISTS case_fees CASCADE;
-- DROP TABLE IF EXISTS fee_schedules CASCADE;

-- Add composite unique constraint to cases table

-- Ensure unique constraint in cases table
ALTER TABLE cases
ADD CONSTRAINT unique_cases_id_filed_date UNIQUE (id, filed_date);

-- Create service_recipients table
CREATE TABLE IF NOT EXISTS service_recipients (
    id BIGSERIAL PRIMARY KEY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    recipient_type TEXT NOT NULL, -- E.g., PARTY, ATTORNEY, etc.
    party_id BIGINT REFERENCES case_parties(id),
    email TEXT NOT NULL,
    service_preference TEXT NOT NULL, -- E.g., ELECTRONIC, MAIL, etc.
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Add unique constraint to service_recipients
ALTER TABLE service_recipients
ADD CONSTRAINT unique_service_recipient UNIQUE (case_id, case_filed_date, recipient_type, party_id, email);


CREATE TABLE IF NOT EXISTS service_transactions (
    id BIGSERIAL PRIMARY KEY,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    recipient_id BIGINT NOT NULL REFERENCES service_recipients(id),
    service_method TEXT NOT NULL, -- e.g., EMAIL, MAIL, etc.
    status TEXT NOT NULL DEFAULT 'PENDING', -- e.g., PENDING, COMPLETED
    delivery_confirmation TEXT, -- e.g., Hash or confirmation code
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add a unique constraint to prevent duplicate transactions
ALTER TABLE service_transactions
ADD CONSTRAINT unique_service_transaction UNIQUE (document_id, recipient_id, service_method);



-- Fee schedules table (master table for all fee types)
CREATE TABLE fee_schedules (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    filing_type TEXT NOT NULL UNIQUE,
    fee_amount NUMERIC(10,2) NOT NULL,
    effective_date DATE NOT NULL,
    end_date DATE,
    waiver_eligible BOOLEAN DEFAULT false,
    description TEXT,
    category TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Case fees table (fees assigned to specific cases)
CREATE TABLE case_fees (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT NOT NULL,
    case_filed_date DATE NOT NULL,
    fee_schedule_id BIGINT REFERENCES fee_schedules(id),
    assigned_to_id BIGINT REFERENCES users(id),
    amount NUMERIC(10,2) NOT NULL,
    due_date DATE,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_by BIGINT REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);
DROP TABLE IF EXISTS financial_transactions CASCADE;
-- Financial transactions table (actual payment records)
CREATE TABLE financial_transactions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    case_id BIGINT,
    case_filed_date DATE,
    case_fee_id BIGINT REFERENCES case_fees(id),
    transaction_type TEXT NOT NULL,
    amount NUMERIC(10,2) NOT NULL,
    payment_method TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    payment_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    payer_id BIGINT REFERENCES users(id),
    received_by BIGINT REFERENCES users(id),
    receipt_number TEXT,
    check_number TEXT,
    routing_number TEXT,
    account_last_four TEXT,
    refund_status TEXT,
    refund_reason TEXT,
    parent_transaction_id BIGINT REFERENCES financial_transactions(id),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- Create all necessary indexes
CREATE INDEX idx_case_fees_case ON case_fees(case_id, case_filed_date);
CREATE INDEX idx_case_fees_assigned_to ON case_fees(assigned_to_id);
CREATE INDEX idx_case_fees_status ON case_fees(status);
CREATE INDEX idx_fee_schedules_type ON fee_schedules(filing_type);
CREATE INDEX idx_financial_transactions_case ON financial_transactions(case_id, case_filed_date);
CREATE INDEX idx_financial_transactions_case_fee ON financial_transactions(case_fee_id);
CREATE INDEX idx_financial_transactions_status ON financial_transactions(status);
CREATE INDEX idx_financial_transactions_payment_date ON financial_transactions(payment_date);

-- Create timestamp triggers
CREATE TRIGGER update_fee_schedules_timestamp
    BEFORE UPDATE ON fee_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_case_fees_timestamp
    BEFORE UPDATE ON case_fees
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_financial_transactions_timestamp
    BEFORE UPDATE ON financial_transactions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();


    CREATE TABLE IF NOT EXISTS api_keys (
        id BIGSERIAL PRIMARY KEY,
        user_id BIGINT NOT NULL REFERENCES users(id),
        key_hash TEXT NOT NULL UNIQUE, -- API key hash
        name TEXT NOT NULL, -- Descriptive name for the API key
        permissions JSONB NOT NULL DEFAULT '{}'::jsonb, -- Permissions for the API key
        status TEXT NOT NULL DEFAULT 'ACTIVE', -- Key status (e.g., ACTIVE, REVOKED)
        expires_at TIMESTAMP WITH TIME ZONE, -- Expiration date
        last_used_at TIMESTAMP WITH TIME ZONE, -- Last usage timestamp
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

    -- Add index to optimize queries on `user_id` and `status`
    CREATE INDEX idx_api_keys_user ON api_keys(user_id);
    CREATE INDEX idx_api_keys_status ON api_keys(status);

    -- Add a trigger to update the `updated_at` timestamp
    CREATE OR REPLACE FUNCTION update_updated_at_timestamp()
    RETURNS TRIGGER AS $$
    BEGIN
        NEW.updated_at = CURRENT_TIMESTAMP;
        RETURN NEW;
    END;
    $$ LANGUAGE plpgsql;

    CREATE TRIGGER update_api_keys_timestamp
        BEFORE UPDATE ON api_keys
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_timestamp();


        -- api_setup.sql
        -- API Tables
        CREATE TABLE IF NOT EXISTS api_requests (
            id BIGSERIAL PRIMARY KEY,
            api_key_id BIGINT REFERENCES api_keys(id),
            endpoint TEXT NOT NULL,
            method TEXT NOT NULL,
            request_body JSONB,
            response_status INTEGER,
            response_body JSONB,
            ip_address INET,
            user_agent TEXT,
            processing_time INTEGER, -- in milliseconds
            error_message TEXT,
            request_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        -- Rate limiting table
        CREATE TABLE IF NOT EXISTS api_rate_limits (
            id BIGSERIAL PRIMARY KEY,
            api_key_id BIGINT REFERENCES api_keys(id),
            window_size INTERVAL NOT NULL,
            max_requests INTEGER NOT NULL,
            current_count INTEGER DEFAULT 0,
            window_start TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            CONSTRAINT unique_api_key_rate_limit UNIQUE (api_key_id)
        );

        -- Add indexes
        CREATE INDEX idx_api_requests_key_id ON api_requests(api_key_id);
        CREATE INDEX idx_api_requests_timestamp ON api_requests(request_timestamp);
        CREATE INDEX idx_api_rate_limits_key_id ON api_rate_limits(api_key_id);
