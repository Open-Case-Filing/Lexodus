-- ##########################################
-- Document Management System
-- ##########################################

-- Document categories and types
CREATE TABLE document_types (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category TEXT NOT NULL, -- e.g., 'PLEADING', 'ORDER', 'MOTION'
    name TEXT NOT NULL,
    description TEXT,
    requires_fee BOOLEAN DEFAULT false,
    requires_service BOOLEAN DEFAULT false,
    requires_judicial_review BOOLEAN DEFAULT false,
    template_path TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_doc_type_name UNIQUE (category, name)
);

-- Main documents table
CREATE TABLE documents (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Document Identification
    case_id BIGINT,
    case_filed_date DATE,
    document_number TEXT,
    document_type_id BIGINT REFERENCES document_types(id),

    -- Document Details
    title TEXT NOT NULL,
    description TEXT,
    page_count INTEGER,
    file_size BIGINT,
    file_format TEXT,

    -- Storage Information
    storage_path TEXT NOT NULL,
    checksum TEXT NOT NULL,

    -- Security and Access Control
    security_level TEXT NOT NULL DEFAULT 'PUBLIC',
    is_sealed BOOLEAN DEFAULT false,
    seal_reason TEXT,
    sealed_by BIGINT REFERENCES users(id),
    sealed_date TIMESTAMP WITH TIME ZONE,

    -- Filing Information
    filed_by BIGINT NOT NULL REFERENCES users(id),
    filed_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_electronic BOOLEAN DEFAULT true,

    -- Metadata
    ocr_status TEXT,
    ocr_completion_date TIMESTAMP WITH TIME ZONE,
    metadata JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

-- Document versions for tracking changes
CREATE TABLE document_versions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    version_number INTEGER NOT NULL,
    storage_path TEXT NOT NULL,
    checksum TEXT NOT NULL,
    file_size BIGINT,

    -- Change Information
    changed_by BIGINT NOT NULL REFERENCES users(id),
    change_reason TEXT,
    change_summary TEXT,

    -- Verification
    verified BOOLEAN DEFAULT false,
    verified_by BIGINT REFERENCES users(id),
    verified_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_doc_version UNIQUE (document_id, version_number)
);

-- Electronic signatures
CREATE TABLE electronic_signatures (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    document_version_id BIGINT NOT NULL REFERENCES document_versions(id),
    signer_id BIGINT NOT NULL REFERENCES users(id),

    -- Signature Details
    signature_type TEXT NOT NULL, -- e.g., 'S-SIGNATURE', 'DIGITAL-CERT'
    signature_image_path TEXT,
    signature_date TIMESTAMP WITH TIME ZONE NOT NULL,
    signature_location TEXT,
    signature_capacity TEXT, -- e.g., 'ATTORNEY', 'JUDGE', 'PARTY'

    -- Verification
    verification_token TEXT NOT NULL,
    verification_method TEXT NOT NULL,
    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Document relationships
CREATE TABLE document_relationships (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    source_document_id BIGINT NOT NULL REFERENCES documents(id),
    target_document_id BIGINT NOT NULL REFERENCES documents(id),
    relationship_type TEXT NOT NULL, -- e.g., 'RESPONSE_TO', 'ATTACHMENT', 'AMENDED_VERSION'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_doc_relationship UNIQUE (source_document_id, target_document_id, relationship_type)
);

-- Document access logs
CREATE TABLE document_access_logs (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    access_type TEXT NOT NULL, -- e.g., 'VIEW', 'DOWNLOAD', 'PRINT'
    access_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ip_address INET,
    user_agent TEXT,
    access_location TEXT,
    success BOOLEAN NOT NULL,
    failure_reason TEXT
);

-- Document processing queue
CREATE TABLE document_processing_queue (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    process_type TEXT NOT NULL, -- e.g., 'OCR', 'VIRUS_SCAN', 'FORMAT_CONVERSION'
    priority INTEGER DEFAULT 5,
    status TEXT NOT NULL DEFAULT 'PENDING',
    attempts INTEGER DEFAULT 0,
    last_attempt TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Indexes for performance
CREATE INDEX idx_documents_case_id_date ON documents(case_id, case_filed_date);
CREATE INDEX idx_documents_document_type ON documents(document_type_id);
CREATE INDEX idx_documents_filed_by ON documents(filed_by);
CREATE INDEX idx_documents_filed_date ON documents(filed_date);
CREATE INDEX idx_document_versions_document_id ON document_versions(document_id);
CREATE INDEX idx_electronic_signatures_document_id ON electronic_signatures(document_id);
CREATE INDEX idx_electronic_signatures_signer ON electronic_signatures(signer_id);
CREATE INDEX idx_document_relationships_source ON document_relationships(source_document_id);
CREATE INDEX idx_document_relationships_target ON document_relationships(target_document_id);
CREATE INDEX idx_document_access_logs_document ON document_access_logs(document_id);
CREATE INDEX idx_document_access_logs_user ON document_access_logs(user_id);
CREATE INDEX idx_document_processing_queue_status ON document_processing_queue(status);

-- Triggers
CREATE OR REPLACE FUNCTION create_initial_document_version()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO document_versions (
        document_id,
        version_number,
        storage_path,
        checksum,
        file_size,
        changed_by
    ) VALUES (
        NEW.id,
        1,
        NEW.storage_path,
        NEW.checksum,
        NEW.file_size,
        NEW.filed_by
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER document_initial_version
    AFTER INSERT ON documents
    FOR EACH ROW
    EXECUTE FUNCTION create_initial_document_version();

-- Add timestamp update triggers
CREATE TRIGGER update_document_types_timestamp
    BEFORE UPDATE ON document_types
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_documents_timestamp
    BEFORE UPDATE ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();
