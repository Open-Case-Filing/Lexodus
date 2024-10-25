-- 1. First recreate tables in correct order
DROP TABLE IF EXISTS financial_transactions CASCADE;
DROP TABLE IF EXISTS case_fees CASCADE;
DROP TABLE IF EXISTS fee_schedules CASCADE;

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
