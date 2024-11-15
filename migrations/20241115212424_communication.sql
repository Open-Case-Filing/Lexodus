-- ##########################################
-- Communication and Collaboration System
-- ##########################################

-- Chat rooms/channels
CREATE TABLE chat_rooms (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Room Details
    name TEXT NOT NULL,
    room_type TEXT NOT NULL, -- e.g., 'CASE', 'DEPARTMENT', 'PRIVATE'
    description TEXT,

    -- Associated Entities
    case_id BIGINT,
    case_filed_date DATE,
    court_id BIGINT REFERENCES courts(id),

    -- Access Control
    is_private BOOLEAN DEFAULT false,
    is_archived BOOLEAN DEFAULT false,
    archived_at TIMESTAMP WITH TIME ZONE,
    archived_by BIGINT REFERENCES users(id),

    -- Metadata
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date),
    CONSTRAINT unique_room_name_type UNIQUE (name, room_type)
);

-- Messages within chat rooms
CREATE TABLE messages (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    chat_room_id BIGINT NOT NULL REFERENCES chat_rooms(id),
    sender_id BIGINT NOT NULL REFERENCES users(id),

    -- Message Content
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'TEXT', -- e.g., 'TEXT', 'HTML', 'MARKDOWN'
    attachments JSONB,

    -- Threading
    parent_message_id BIGINT REFERENCES messages(id),
    thread_id BIGINT,

    -- Status
    is_edited BOOLEAN DEFAULT false,
    edited_at TIMESTAMP WITH TIME ZONE,
    is_deleted BOOLEAN DEFAULT false,
    deleted_at TIMESTAMP WITH TIME ZONE,

    -- Metadata
    sent_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Chat room participants
CREATE TABLE chat_participants (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    chat_room_id BIGINT NOT NULL REFERENCES chat_rooms(id),
    user_id BIGINT NOT NULL REFERENCES users(id),

    -- Participation Details
    role TEXT NOT NULL DEFAULT 'MEMBER', -- e.g., 'ADMIN', 'MEMBER', 'GUEST'
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    invited_by BIGINT REFERENCES users(id),

    -- Status
    is_active BOOLEAN DEFAULT true,
    last_read_message_id BIGINT REFERENCES messages(id),
    muted_until TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_chat_participant UNIQUE (chat_room_id, user_id)
);

-- Message reactions
CREATE TABLE message_reactions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    message_id BIGINT NOT NULL REFERENCES messages(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    reaction TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_message_reaction UNIQUE (message_id, user_id, reaction)
);

-- Notification templates
CREATE TABLE notification_templates (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    subject_template TEXT NOT NULL,
    body_template TEXT NOT NULL,
    category TEXT NOT NULL,
    format TEXT NOT NULL DEFAULT 'TEXT', -- e.g., 'TEXT', 'HTML'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Notifications
CREATE TABLE notifications (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Recipient Information
    user_id BIGINT NOT NULL REFERENCES users(id),
    template_id BIGINT REFERENCES notification_templates(id),

    -- Notification Content
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    notification_type TEXT NOT NULL, -- e.g., 'SYSTEM', 'CASE', 'DOCUMENT'
    priority TEXT NOT NULL DEFAULT 'NORMAL', -- e.g., 'HIGH', 'NORMAL', 'LOW'

    -- Reference Entities
    case_id BIGINT,
    case_filed_date DATE,
    document_id BIGINT REFERENCES documents(id),
    event_id BIGINT REFERENCES case_events(id),

    -- Status
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMP WITH TIME ZONE,
    is_archived BOOLEAN DEFAULT false,
    archived_at TIMESTAMP WITH TIME ZONE,

    -- Delivery Status
    delivery_status TEXT NOT NULL DEFAULT 'PENDING',
    delivery_attempts INTEGER DEFAULT 0,
    last_attempt_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date)
);

-- User presence tracking
CREATE TABLE user_presence (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL, -- e.g., 'ONLINE', 'AWAY', 'BUSY', 'OFFLINE'
    last_active_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    device_info JSONB,
    ip_address INET,
    location TEXT,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_presence UNIQUE (user_id)
);

-- Indexes for performance
CREATE INDEX idx_chat_rooms_case_id ON chat_rooms(case_id, case_filed_date);
CREATE INDEX idx_messages_chat_room ON messages(chat_room_id);
CREATE INDEX idx_messages_sender ON messages(sender_id);
CREATE INDEX idx_messages_thread ON messages(thread_id);
CREATE INDEX idx_chat_participants_room ON chat_participants(chat_room_id);
CREATE INDEX idx_chat_participants_user ON chat_participants(user_id);
CREATE INDEX idx_message_reactions_message ON message_reactions(message_id);
CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_case ON notifications(case_id, case_filed_date);
CREATE INDEX idx_user_presence_status ON user_presence(status);

-- Triggers
CREATE OR REPLACE FUNCTION update_thread_id()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_message_id IS NOT NULL THEN
        NEW.thread_id := COALESCE(
            (SELECT thread_id FROM messages WHERE id = NEW.parent_message_id),
            NEW.parent_message_id
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_message_thread_id
    BEFORE INSERT ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_thread_id();

-- Add timestamp update triggers
CREATE TRIGGER update_chat_rooms_timestamp
    BEFORE UPDATE ON chat_rooms
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_messages_timestamp
    BEFORE UPDATE ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_chat_participants_timestamp
    BEFORE UPDATE ON chat_participants
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_notification_templates_timestamp
    BEFORE UPDATE ON notification_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();

CREATE TRIGGER update_user_presence_timestamp
    BEFORE UPDATE ON user_presence
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_timestamp();
