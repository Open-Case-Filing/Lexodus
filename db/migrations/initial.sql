-- CM/ECF Complete Schema

-- Drop existing tables
DROP TABLE IF EXISTS chat_audit_logs CASCADE;
DROP TABLE IF EXISTS message_reactions CASCADE;
DROP TABLE IF EXISTS message_threads CASCADE;
DROP TABLE IF EXISTS user_presence CASCADE;
DROP TABLE IF EXISTS chat_files CASCADE;
DROP TABLE IF EXISTS chat_notifications CASCADE;
DROP TABLE IF EXISTS chat_participants CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS chat_rooms CASCADE;
DROP TABLE IF EXISTS user_last_actions CASCADE;
DROP TABLE IF EXISTS user_frequent_actions CASCADE;
DROP TABLE IF EXISTS user_actions CASCADE;
DROP TABLE IF EXISTS cases_summary CASCADE;
DROP TABLE IF EXISTS fees_payments CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;
DROP TABLE IF EXISTS document_versions CASCADE;
DROP TABLE IF EXISTS case_history CASCADE;
DROP TABLE IF EXISTS activity_logs CASCADE;
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS appeals CASCADE;
DROP TABLE IF EXISTS judgments CASCADE;
DROP TABLE IF EXISTS pretrial_conferences CASCADE;
DROP TABLE IF EXISTS motions CASCADE;
DROP TABLE IF EXISTS pleadings CASCADE;
DROP TABLE IF EXISTS discovery CASCADE;
DROP TABLE IF EXISTS scheduling_orders CASCADE;
DROP TABLE IF EXISTS entity_relationships CASCADE;
DROP TABLE IF EXISTS case_documents CASCADE;
DROP TABLE IF EXISTS case_parties CASCADE;
DROP TABLE IF EXISTS dockets CASCADE;
DROP TABLE IF EXISTS parties CASCADE;
DROP TABLE IF EXISTS hearings CASCADE;
DROP TABLE IF EXISTS documents CASCADE;
DROP TABLE IF EXISTS case_transfers CASCADE;
DROP TABLE IF EXISTS cases CASCADE;
DROP TABLE IF EXISTS judges CASCADE;
DROP TABLE IF EXISTS courts CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS roles CASCADE;

-- Create tables
CREATE TABLE roles (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL UNIQUE,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE users (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username text NOT NULL UNIQUE,
  password_hash text NOT NULL,
  role_id bigint NOT NULL REFERENCES roles(id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_users_role_id ON users (role_id);

CREATE TABLE courts (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL UNIQUE,
  district text NOT NULL,
  circuit text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE judges (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL,
  court_id bigint NOT NULL REFERENCES courts(id),
  birthdate date NOT NULL,
  appointed_date date NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_judges_court_id ON judges (court_id);

CREATE TABLE cases (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_number text NOT NULL UNIQUE,
  title text NOT NULL,
  status text NOT NULL,
  filed_date date NOT NULL,
  closed_date date,
  court_id bigint NOT NULL REFERENCES courts(id),
  current_court_id bigint REFERENCES courts(id),
  judge_id bigint REFERENCES judges(id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
) PARTITION BY RANGE (filed_date);

CREATE TABLE cases_2024 PARTITION OF cases
  FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');

CREATE TABLE cases_2025 PARTITION OF cases
  FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');

CREATE INDEX ON cases_2024 (case_number);
CREATE INDEX ON cases_2025 (case_number);
CREATE INDEX idx_cases_court_id ON cases (court_id);
CREATE INDEX idx_cases_current_court_id ON cases (current_court_id);
CREATE INDEX idx_cases_judge_id ON cases (judge_id);

CREATE TABLE case_transfers (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases(id),
  from_court_id bigint REFERENCES courts(id),
  to_court_id bigint REFERENCES courts(id),
  transfer_date date NOT NULL,
  transfer_reason text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_case_transfers_case_id ON case_transfers (case_id);
CREATE INDEX idx_case_transfers_from_court_id ON case_transfers (from_court_id);
CREATE INDEX idx_case_transfers_to_court_id ON case_transfers (to_court_id);

CREATE TABLE documents (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  title text NOT NULL,
  file_path text NOT NULL,
  filed_date date NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE hearings (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  hearing_date timestamp with time zone NOT NULL,
  location text NOT NULL,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE parties (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  name text NOT NULL,
  role text NOT NULL,
  attorney_id bigint REFERENCES users (id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE dockets (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  entry_date date NOT NULL,
  description text NOT NULL,
  document_id bigint REFERENCES documents (id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE case_parties (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  party_id bigint REFERENCES parties (id),
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE case_documents (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  document_id bigint REFERENCES documents (id),
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE entity_relationships (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  entity1_id bigint NOT NULL,
  entity2_id bigint NOT NULL,
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_case_parties_case_id ON case_parties (case_id);
CREATE INDEX idx_case_documents_case_id ON case_documents (case_id);
CREATE INDEX idx_entity_relationships_entity1_id ON entity_relationships (entity1_id);
CREATE INDEX idx_entity_relationships_entity2_id ON entity_relationships (entity2_id);

CREATE TABLE scheduling_orders (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  order_date date NOT NULL,
  description text NOT NULL,
  deadline date NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE discovery (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  request_type text NOT NULL,
  request_date date NOT NULL,
  response_date date,
  status text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE pleadings (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  pleading_type text NOT NULL,
  filed_date date NOT NULL,
  status text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE motions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  motion_type text NOT NULL,
  filed_date date NOT NULL,
  status text NOT NULL,
  outcome text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE pretrial_conferences (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  conference_date timestamp with time zone NOT NULL,
  location text NOT NULL,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE judgments (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  judgment_date date NOT NULL,
  description text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE appeals (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  case_transfer_id bigint REFERENCES case_transfers(id),
  appeal_date date NOT NULL,
  appellate_court text NOT NULL,
  outcome text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_appeals_case_id ON appeals (case_id);
CREATE INDEX idx_appeals_case_transfer_id ON appeals (case_transfer_id);

CREATE TABLE notifications (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  message text NOT NULL,
  notification_date timestamp with time zone DEFAULT now(),
  is_read boolean DEFAULT false,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE activity_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE case_history (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  change_date timestamp with time zone DEFAULT now(),
  changed_by bigint REFERENCES users (id),
  change_description text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE document_versions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  document_id bigint REFERENCES documents (id),
  version_number int NOT NULL,
  file_path text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE tasks (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  assigned_to bigint REFERENCES users (id),
  task_description text NOT NULL,
  due_date date NOT NULL,
  status text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE fees_payments (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases(id),
  amount numeric(10, 2) NOT NULL,
  payment_date date NOT NULL,
  payment_method text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE MATERIALIZED VIEW cases_summary AS
SELECT
  id,
  case_number,
  title,
  status,
  filed_date
FROM
  cases;

CREATE INDEX idx_cases_summary_case_number ON cases_summary (case_number);

CREATE TABLE user_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE user_frequent_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  frequency int NOT NULL,
  last_used timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE user_last_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  last_used timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_user_actions_user_id ON user_actions (user_id);
CREATE INDEX idx_user_frequent_actions_user_id ON user_frequent_actions (user_id);
CREATE INDEX idx_user_last_actions_user_id ON user_last_actions (user_id);

CREATE TABLE chat_rooms (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint REFERENCES cases (id),
  name text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE messages (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  content text NOT NULL,
  sent_at timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE chat_participants (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  joined_at timestamp with time zone DEFAULT now(),
  left_at timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_chat_rooms_case_id ON chat_rooms (case_id);
CREATE INDEX idx_messages_chat_room_id ON messages (chat_room_id);
CREATE INDEX idx_chat_participants_chat_room_id ON chat_participants (chat_room_id);

CREATE TABLE chat_notifications (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  chat_room_id bigint REFERENCES chat_rooms (id),
  message_id bigint REFERENCES messages (id),
  is_read boolean DEFAULT false,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE chat_files (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  file_path text NOT NULL,
  uploaded_at timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE user_presence (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  status text NOT NULL,
  last_active timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE message_threads (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  parent_message_id bigint REFERENCES messages (id),
  child_message_id bigint REFERENCES messages (id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE message_reactions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  message_id bigint REFERENCES messages (id),
  user_id bigint REFERENCES users (id),
  reaction text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE chat_audit_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  action text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_chat_notifications_user_id ON chat_notifications (user_id);
CREATE INDEX idx_chat_files_chat_room_id ON chat_files (chat_room_id);
CREATE INDEX idx_user_presence_user_id ON user_presence (user_id);
CREATE INDEX idx_message_threads_parent_message_id ON message_threads (parent_message_id);
CREATE INDEX idx_message_reactions_message_id ON message_reactions (message_id);
CREATE INDEX idx_chat_audit_logs_chat_room_id ON chat_audit_logs (chat_room_id);

-- Views
CREATE OR REPLACE VIEW case_history_with_transfers AS
SELECT
  c.id AS case_id,
  c.case_number,
  c.title,
  ct.transfer_date,
  fc.name AS from_court,
  tc.name AS to_court,
  ct.transfer_reason
FROM
  cases c
LEFT JOIN
  case_transfers ct ON c.id = ct.case_id
LEFT JOIN
  courts fc ON ct.from_court_id = fc.id
LEFT JOIN
  courts tc ON ct.to_court_id = tc.id
ORDER BY
  c.id, ct.transfer_date;

-- Functions
CREATE OR REPLACE FUNCTION transfer_case(
  p_case_id bigint,
  p_to_court_id bigint,
  p_transfer_reason text
) RETURNS void AS $$
DECLARE
  v_from_court_id bigint;
BEGIN
  -- Get the current court of the case
  SELECT current_court_id INTO v_from_court_id
  FROM cases
  WHERE id = p_case_id;

  -- Insert a new record into case_transfers
  INSERT INTO case_transfers (case_id, from_court_id, to_court_id, transfer_date, transfer_reason)
  VALUES (p_case_id, v_from_court_id, p_to_court_id, CURRENT_DATE, p_transfer_reason);

  -- Update the case's current court
  UPDATE cases
  SET current_court_id = p_to_court_id,
      updated_at = now()
  WHERE id = p_case_id;
END;
$$ LANGUAGE plpgsql;

-- Triggers
CREATE OR REPLACE FUNCTION create_appeal_transfer() RETURNS TRIGGER AS $$
DECLARE
  v_appellate_court_id bigint;
  v_transfer_id bigint;
BEGIN
  -- Get the ID of the appellate court
  SELECT id INTO v_appellate_court_id
  FROM courts
  WHERE name = NEW.appellate_court;

  -- Create a case transfer
  INSERT INTO case_transfers (case_id, from_court_id, to_court_id, transfer_date, transfer_reason)
  VALUES (NEW.case_id,
          (SELECT current_court_id FROM cases WHERE id = NEW.case_id),
          v_appellate_court_id,
          NEW.appeal_date,
          'Appeal filed')
  RETURNING id INTO v_transfer_id;

  -- Update the appeals record with the transfer ID
  NEW.case_transfer_id := v_transfer_id;

  -- Update the case's current court
  UPDATE cases
  SET current_court_id = v_appellate_court_id,
      updated_at = now()
  WHERE id = NEW.case_id;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER appeal_transfer_trigger
BEFORE INSERT ON appeals
FOR EACH ROW
EXECUTE FUNCTION create_appeal_transfer();
