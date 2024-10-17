-- Lexodus Complete Schema for Testing

-- Drop existing tables (if they exist)
DROP TABLE IF EXISTS electronic_signatures CASCADE;
DROP TABLE IF EXISTS document_access_logs CASCADE;
DROP TABLE IF EXISTS case_schedules CASCADE;
DROP TABLE IF EXISTS legal_citations CASCADE;
DROP TABLE IF EXISTS case_status_history CASCADE;
DROP TABLE IF EXISTS case_flags CASCADE;
DROP TABLE IF EXISTS document_versions CASCADE;
DROP TABLE IF EXISTS court_calendar CASCADE;
DROP TABLE IF EXISTS sealed_documents CASCADE;
DROP TABLE IF EXISTS user_activity_logs CASCADE;
DROP TABLE IF EXISTS related_cases CASCADE;
DROP TABLE IF EXISTS case_tags CASCADE;
DROP TABLE IF EXISTS case_assignments CASCADE;
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
DROP TABLE IF EXISTS document_seal_history CASCADE;
DROP TABLE IF EXISTS sealed_document_access_logs CASCADE;
DROP TABLE IF EXISTS persons CASCADE;

-- Create tables

CREATE TABLE IF NOT EXISTS roles (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL UNIQUE,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

-- New table for permissions
CREATE TABLE IF NOT EXISTS permissions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL UNIQUE,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

-- New junction table for role-permission relationships
CREATE TABLE IF NOT EXISTS role_permissions (
  role_id bigint REFERENCES roles(id),
  permission_id bigint REFERENCES permissions(id),
  PRIMARY KEY (role_id, permission_id)
);


CREATE TABLE IF NOT EXISTS users (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username text NOT NULL UNIQUE,
  password_hash text NOT NULL,
  role_id bigint NOT NULL REFERENCES roles(id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_users_role_id ON users (role_id);

CREATE TABLE IF NOT EXISTS courts (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL,
  district text NOT NULL,
  circuit text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  CONSTRAINT unique_court_name_district UNIQUE (name, district)
);

CREATE TABLE IF NOT EXISTS judges (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL UNIQUE,
  court_id bigint NOT NULL REFERENCES courts(id),
  birthdate text NOT NULL,
  appointed_date text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX idx_judges_court_id ON judges (court_id);

-- Create the cases table (partitioned)
CREATE TABLE IF NOT EXISTS cases (
  id bigint GENERATED ALWAYS AS IDENTITY,
  case_number text NOT NULL,
  title text NOT NULL,
  status text NOT NULL,
  filed_date text NOT NULL,
  closed_date text,
  court_id bigint NOT NULL REFERENCES courts(id),
  current_court_id bigint REFERENCES courts(id),
  judge_id bigint REFERENCES judges(id),
  docket_id bigint,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  PRIMARY KEY (id, filed_date)
) PARTITION BY RANGE (filed_date);

-- Create partitions for cases
CREATE TABLE cases_2019 PARTITION OF cases
FOR VALUES FROM ('2019-01-01') TO ('2020-01-01');

CREATE TABLE cases_2020 PARTITION OF cases
FOR VALUES FROM ('2020-01-01') TO ('2021-01-01');

CREATE TABLE cases_2021 PARTITION OF cases
FOR VALUES FROM ('2021-01-01') TO ('2022-01-01');

CREATE TABLE cases_2022 PARTITION OF cases
FOR VALUES FROM ('2022-01-01') TO ('2023-01-01');

CREATE TABLE cases_2023 PARTITION OF cases
FOR VALUES FROM ('2023-01-01') TO ('2024-01-01');

CREATE TABLE cases_2024 PARTITION OF cases
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');

CREATE TABLE cases_2025 PARTITION OF cases
FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');

-- Create the unique indexes for cases
CREATE UNIQUE INDEX idx_cases_case_number_filed_date ON cases (case_number, filed_date);

CREATE TABLE IF NOT EXISTS dockets (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_number text NOT NULL,
  entry_number bigint NOT NULL,
  entry_date text NOT NULL,
  entry_text text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date),
  CONSTRAINT unique_docket_number UNIQUE (docket_number)
);

CREATE INDEX idx_dockets_case_id_filed_date ON dockets (case_id, case_filed_date);

CREATE TABLE IF NOT EXISTS case_history (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  change_date timestamp with time zone DEFAULT now(),
  changed_by bigint REFERENCES users(id),
  change_description text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS case_transfers (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  from_court_id bigint NOT NULL REFERENCES courts(id),
  to_court_id bigint NOT NULL REFERENCES courts(id),
  transfer_date text NOT NULL,
  transfer_reason text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE UNIQUE INDEX idx_case_transfers_unique ON case_transfers (case_id, filed_date, transfer_date);

CREATE TABLE IF NOT EXISTS documents (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  title text NOT NULL,
  file_path text NOT NULL,
  filed_date text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS hearings (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  hearing_date timestamp with time zone NOT NULL,
  location text NOT NULL,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS parties (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  name text NOT NULL,
  role text NOT NULL,
  attorney_id bigint REFERENCES users (id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS case_parties (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  party_id bigint REFERENCES parties (id),
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS case_documents (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  document_id bigint REFERENCES documents (id),
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS entity_relationships (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  entity1_id bigint NOT NULL,
  entity2_id bigint NOT NULL,
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS scheduling_orders (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  order_date text NOT NULL,
  description text NOT NULL,
  deadline text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS discovery (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  request_type text NOT NULL,
  request_date text NOT NULL,
  response_date text,
  status text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS pleadings (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  pleading_type text NOT NULL,
  filed_date text NOT NULL,
  status text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS motions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  motion_type text NOT NULL,
  filed_date text NOT NULL,
  status text NOT NULL,
  outcome text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS pretrial_conferences (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  conference_date timestamp with time zone NOT NULL,
  location text NOT NULL,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS judgments (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  docket_id bigint REFERENCES dockets(id),
  judgment_date text NOT NULL,
  description text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS appeals (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  case_filed_date text NOT NULL,
  case_transfer_id bigint REFERENCES case_transfers(id),
  appeal_date text NOT NULL,
  appellate_court text NOT NULL,
  outcome text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases (id, filed_date)
);

CREATE TABLE IF NOT EXISTS notifications (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  message text NOT NULL,
  notification_date timestamp with time zone DEFAULT now(),
  is_read boolean DEFAULT false,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS activity_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS fees_payments (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  amount numeric(10, 2) NOT NULL,
  payment_date text NOT NULL,
  payment_method text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS user_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_frequent_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  frequency int NOT NULL,
  last_used timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_last_actions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  action_type text NOT NULL,
  last_used timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS chat_rooms (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint,
  case_filed_date text,
  name text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, case_filed_date) REFERENCES cases(id, filed_date),
  CONSTRAINT unique_chat_room_name UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS messages (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  content text NOT NULL,
  sent_at timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS chat_participants (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  joined_at timestamp with time zone DEFAULT now(),
  left_at timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS chat_notifications (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  chat_room_id bigint REFERENCES chat_rooms (id),
  message_id bigint REFERENCES messages (id),
  is_read boolean DEFAULT false,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS chat_files (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  file_path text NOT NULL,
  uploaded_at timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_presence (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint REFERENCES users (id),
  status text NOT NULL,
  last_active timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS message_threads (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  parent_message_id bigint REFERENCES messages (id),
  child_message_id bigint REFERENCES messages (id),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS message_reactions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  message_id bigint REFERENCES messages (id),
  user_id bigint REFERENCES users (id),
  reaction text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS chat_audit_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  chat_room_id bigint REFERENCES chat_rooms (id),
  user_id bigint REFERENCES users (id),
  action text NOT NULL,
  action_date timestamp with time zone DEFAULT now(),
  details text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS case_assignments (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  user_id bigint NOT NULL,
  assignment_type text NOT NULL,
  assigned_date text NOT NULL,
  end_date text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS case_tags (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  tag text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS related_cases (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  related_case_id bigint NOT NULL,
  related_case_filed_date text NOT NULL,
  relationship_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date),
  FOREIGN KEY (related_case_id, related_case_filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS user_activity_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint NOT NULL,
  action_type text NOT NULL,
  action_details jsonb,
  ip_address inet,
  user_agent text,
  created_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS electronic_signatures (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint NOT NULL REFERENCES users(id),
  document_id bigint NOT NULL REFERENCES documents(id),
  signature_date timestamp with time zone DEFAULT now(),
  signature_hash text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS document_access_logs (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id bigint NOT NULL REFERENCES users(id),
  document_id bigint NOT NULL REFERENCES documents(id),
  access_type text NOT NULL,
  access_date timestamp with time zone DEFAULT now(),
  ip_address inet,
  user_agent text
);

CREATE TABLE IF NOT EXISTS case_schedules (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  event_type text NOT NULL,
  event_date timestamp with time zone NOT NULL,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS legal_citations (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  document_id bigint NOT NULL REFERENCES documents(id),
  citation_text text NOT NULL,
  citation_type text NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS case_status_history (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  status text NOT NULL,
  effective_date timestamp with time zone DEFAULT now(),
  changed_by bigint REFERENCES users(id),
  created_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS case_flags (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  case_id bigint NOT NULL,
  filed_date text NOT NULL,
  flag_type text NOT NULL,
  flag_value boolean NOT NULL,
  set_by bigint REFERENCES users(id),
  set_date timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS document_versions (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  document_id bigint NOT NULL REFERENCES documents(id),
  version_number int NOT NULL,
  file_path text NOT NULL,
  uploaded_by bigint REFERENCES users(id),
  upload_date timestamp with time zone DEFAULT now(),
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS court_calendar (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  court_id bigint NOT NULL REFERENCES courts(id),
  event_type text NOT NULL,
  event_date timestamp with time zone NOT NULL,
  case_id bigint,
  filed_date text,
  description text,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),
  FOREIGN KEY (case_id, filed_date) REFERENCES cases(id, filed_date)
);

CREATE TABLE IF NOT EXISTS sealed_documents (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  document_id bigint NOT NULL REFERENCES documents(id),
  sealed_by bigint REFERENCES users(id),
  seal_date timestamp with time zone DEFAULT now(),
  seal_reason text,
  unsealed_by bigint REFERENCES users(id),
  unseal_date timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

-- Create indexes
CREATE INDEX idx_documents_case_id_filed_date ON documents (case_id, case_filed_date);
CREATE INDEX idx_hearings_case_id_filed_date ON hearings (case_id, case_filed_date);
CREATE INDEX idx_parties_case_id_filed_date ON parties (case_id, case_filed_date);
CREATE INDEX idx_case_parties_case_id_filed_date ON case_parties (case_id, case_filed_date);
CREATE INDEX idx_case_documents_case_id_filed_date ON case_documents (case_id, case_filed_date);
CREATE INDEX idx_entity_relationships_entity1_id ON entity_relationships (entity1_id);
CREATE INDEX idx_entity_relationships_entity2_id ON entity_relationships (entity2_id);
CREATE INDEX idx_scheduling_orders_case_id_filed_date ON scheduling_orders (case_id, case_filed_date);
CREATE INDEX idx_discovery_case_id_filed_date ON discovery (case_id, case_filed_date);
CREATE INDEX idx_pleadings_case_id_filed_date ON pleadings (case_id, case_filed_date);
CREATE INDEX idx_motions_case_id_filed_date ON motions (case_id, case_filed_date);
CREATE INDEX idx_pretrial_conferences_case_id_filed_date ON pretrial_conferences (case_id, case_filed_date);
CREATE INDEX idx_judgments_case_id_filed_date ON judgments (case_id, case_filed_date);
CREATE INDEX idx_appeals_case_transfer_id ON appeals (case_transfer_id);
CREATE INDEX idx_user_actions_user_id ON user_actions (user_id);
CREATE INDEX idx_user_frequent_actions_user_id ON user_frequent_actions (user_id);
CREATE INDEX idx_user_last_actions_user_id ON user_last_actions (user_id);
CREATE INDEX idx_chat_rooms_case_id ON chat_rooms (case_id);
CREATE INDEX idx_messages_chat_room_id ON messages (chat_room_id);
CREATE INDEX idx_chat_participants_chat_room_id ON chat_participants (chat_room_id);
CREATE INDEX idx_chat_notifications_user_id ON chat_notifications (user_id);
CREATE INDEX idx_chat_files_chat_room_id ON chat_files (chat_room_id);
CREATE INDEX idx_user_presence_user_id ON user_presence (user_id);
CREATE INDEX idx_message_threads_parent_message_id ON message_threads (parent_message_id);
CREATE INDEX idx_message_reactions_message_id ON message_reactions (message_id);
CREATE INDEX idx_chat_audit_logs_chat_room_id ON chat_audit_logs (chat_room_id);
CREATE INDEX idx_case_assignments_case_id_filed_date ON case_assignments (case_id, filed_date);
CREATE INDEX idx_case_assignments_user_id ON case_assignments (user_id);
CREATE INDEX idx_case_tags_case_id_filed_date ON case_tags (case_id, filed_date);
CREATE INDEX idx_case_tags_tag ON case_tags (tag);
CREATE INDEX idx_related_cases_case_id_filed_date ON related_cases (case_id, filed_date);
CREATE INDEX idx_related_cases_related_case_id_filed_date ON related_cases (related_case_id, related_case_filed_date);
CREATE INDEX idx_user_activity_logs_user_id ON user_activity_logs (user_id);
CREATE INDEX idx_user_activity_logs_action_type ON user_activity_logs (action_type);
CREATE INDEX idx_user_activity_logs_created_at ON user_activity_logs (created_at);
CREATE INDEX idx_electronic_signatures_user_id ON electronic_signatures (user_id);
CREATE INDEX idx_electronic_signatures_document_id ON electronic_signatures (document_id);
CREATE INDEX idx_document_access_logs_user_id ON document_access_logs (user_id);
CREATE INDEX idx_document_access_logs_document_id ON document_access_logs (document_id);
CREATE INDEX idx_case_schedules_case_id_filed_date ON case_schedules (case_id, filed_date);
CREATE INDEX idx_legal_citations_document_id ON legal_citations (document_id);
CREATE INDEX idx_case_status_history_case_id_filed_date ON case_status_history (case_id, filed_date);
CREATE INDEX idx_case_flags_case_id_filed_date ON case_flags (case_id, filed_date);
CREATE INDEX idx_document_versions_document_id ON document_versions (document_id);
CREATE INDEX idx_court_calendar_court_id ON court_calendar (court_id);
CREATE INDEX idx_court_calendar_case_id_filed_date ON court_calendar (case_id, filed_date);
CREATE INDEX idx_sealed_documents_document_id ON sealed_documents (document_id);

-- Create functions
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now();
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE FUNCTION transfer_case(
  p_case_id bigint,
  p_to_court_id bigint,
  p_transfer_reason text
) RETURNS void AS $$
DECLARE
  v_from_court_id bigint;
  v_filed_date text;
BEGIN
  -- Get the current court of the case and filed date
  SELECT current_court_id, filed_date INTO v_from_court_id, v_filed_date
  FROM cases
  WHERE id = p_case_id;

  -- Insert a new record into case_transfers
  INSERT INTO case_transfers (case_id, filed_date, from_court_id, to_court_id, transfer_date, transfer_reason)
  VALUES (p_case_id, v_filed_date, v_from_court_id, p_to_court_id, CURRENT_DATE, p_transfer_reason);

  -- Update the case's current court
  UPDATE cases
  SET current_court_id = p_to_court_id,
      updated_at = now()
  WHERE id = p_case_id;
END;
$$ LANGUAGE plpgsql;

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
  INSERT INTO case_transfers (case_id, filed_date, from_court_id, to_court_id, transfer_date, transfer_reason)
  VALUES (NEW.case_id, NEW.case_filed_date,
      (SELECT current_court_id FROM cases WHERE id = NEW.case_id),
      v_appellate_court_id,
      NEW.appeal_date,
      'Appeal to ' || NEW.appellate_court)
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

CREATE OR REPLACE FUNCTION log_user_activity() RETURNS TRIGGER AS $$
DECLARE
  action_type text;
  action_details jsonb;
  current_user_id bigint;
  current_ip_address inet;
  current_user_agent text;
BEGIN
  -- Determine the action type based on the operation and table
  action_type := TG_OP || '_' || TG_TABLE_NAME;

  -- Get the current user context
  -- These values should be set by the application before performing database operations
  current_user_id := current_setting('my.application_user_id', true)::bigint;
  current_ip_address := inet(current_setting('my.application_ip_address', true));
  current_user_agent := current_setting('my.application_user_agent', true);

  -- Create a JSON object with the details of the action
  action_details := jsonb_build_object(
    'table', TG_TABLE_NAME,
    'operation', TG_OP,
    'time', now(),
    'new_data', row_to_json(NEW),
    'old_data', CASE WHEN TG_OP = 'DELETE' THEN row_to_json(OLD) ELSE null END,
    'current_user_id', current_user_id,
    'current_ip_address', current_ip_address,
    'current_user_agent', current_user_agent
  );

  -- Only log the activity if we have a valid user ID
  IF current_user_id IS NOT NULL AND EXISTS (SELECT 1 FROM users WHERE id = current_user_id) THEN
    INSERT INTO user_activity_logs (user_id, action_type, action_details, ip_address, user_agent)
    VALUES (
      current_user_id,
      action_type,
      action_details,
      current_ip_address,
      current_user_agent
    );
  ELSE
    -- Log to PostgreSQL's log that we couldn't record this activity
    RAISE NOTICE 'Could not log user activity: invalid user_id %', current_user_id;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_user_context(p_user_id bigint, p_ip_address inet, p_user_agent text) RETURNS void AS $$
BEGIN
  PERFORM set_config('my.application_user_id', p_user_id::text, false);
  PERFORM set_config('my.application_ip_address', p_ip_address::text, false);
  PERFORM set_config('my.application_user_agent', p_user_agent, false);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_case_status() RETURNS TRIGGER AS $$
BEGIN
  IF OLD.status IS DISTINCT FROM NEW.status THEN
    INSERT INTO case_status_history (case_id, filed_date, status, effective_date, changed_by)
    VALUES (NEW.id, NEW.filed_date, NEW.status, now(), current_setting('app.current_user_id')::bigint);
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_document_version() RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO document_versions (document_id, version_number, file_path, uploaded_by)
  VALUES (NEW.id,
          COALESCE((SELECT MAX(version_number) FROM document_versions WHERE document_id = NEW.id), 0) + 1,
          NEW.file_path,
          current_setting('app.current_user_id')::bigint);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers
CREATE TRIGGER update_timestamp
BEFORE UPDATE ON cases
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER appeal_transfer_trigger
BEFORE INSERT ON appeals
FOR EACH ROW EXECUTE FUNCTION create_appeal_transfer();

CREATE TRIGGER log_cases_activity
AFTER INSERT OR UPDATE OR DELETE ON cases
FOR EACH ROW EXECUTE FUNCTION log_user_activity();

CREATE TRIGGER log_documents_activity
AFTER INSERT OR UPDATE OR DELETE ON documents
FOR EACH ROW EXECUTE FUNCTION log_user_activity();

CREATE TRIGGER log_hearings_activity
AFTER INSERT OR UPDATE OR DELETE ON hearings
FOR EACH ROW EXECUTE FUNCTION log_user_activity();

CREATE TRIGGER case_status_change_trigger
AFTER UPDATE OF status ON cases
FOR EACH ROW EXECUTE FUNCTION update_case_status();

CREATE TRIGGER document_version_trigger
AFTER INSERT OR UPDATE OF file_path ON documents
FOR EACH ROW EXECUTE FUNCTION create_document_version();

-- Create views
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

CREATE OR REPLACE VIEW user_activity_summary AS
SELECT
  u.username,
  ual.action_type,
  COUNT(*) as action_count,
  MIN(ual.created_at) as first_action,
  MAX(ual.created_at) as last_action
FROM
  user_activity_logs ual
JOIN
  users u ON ual.user_id = u.id
GROUP BY
  u.username, ual.action_type
ORDER BY
  u.username, action_count DESC;

CREATE OR REPLACE VIEW full_case_history AS
SELECT
  c.id AS case_id,
  c.case_number,
  c.title,
  d.docket_number,
  d.entry_number,
  d.entry_date,
  d.entry_text,
  ch.change_date,
  ch.changed_by,
  ch.change_description
FROM
  cases c
LEFT JOIN dockets d ON c.id = d.case_id
LEFT JOIN case_history ch ON c.id = ch.case_id
ORDER BY
  c.id, d.entry_date, ch.change_date;

CREATE OR REPLACE VIEW case_summary AS
SELECT
  c.id AS case_id,
  c.case_number,
  c.title,
  c.status,
  c.filed_date,
  c.closed_date,
  co.name AS court_name,
  j.name AS judge_name,
  (SELECT COUNT(*) FROM documents WHERE case_id = c.id) AS document_count,
  (SELECT COUNT(*) FROM hearings WHERE case_id = c.id) AS hearing_count,
  (SELECT COUNT(*) FROM motions WHERE case_id = c.id) AS motion_count,
  (SELECT status FROM case_status_history WHERE case_id = c.id ORDER BY effective_date DESC LIMIT 1) AS latest_status,
  (SELECT array_agg(DISTINCT flag_type) FROM case_flags WHERE case_id = c.id AND flag_value = true) AS active_flags
FROM
  cases c
JOIN courts co ON c.court_id = co.id
LEFT JOIN judges j ON c.judge_id = j.id;

-- Sample data insertion (minimal set for testing)
INSERT INTO roles (name, description) VALUES
('judge', 'Federal judge'),
('attorney', 'Practicing attorney'),
('clerk', 'Court clerk'),
('party', 'A party to a case'),
('admin', 'System administrator');

INSERT INTO users (username, password_hash, role_id) VALUES
('judge1', 'hash1', (SELECT id FROM roles WHERE name = 'judge')),
('attorney1', 'hash2', (SELECT id FROM roles WHERE name = 'attorney')),
('clerk1', 'hash3', (SELECT id FROM roles WHERE name = 'clerk')),
('admin1', 'hash4', (SELECT id FROM roles WHERE name = 'admin'));

INSERT INTO courts (name, district, circuit) VALUES
('U.S. District Court for the Southern District of New York', 'Southern District of New York', 'Second Circuit'),
('U.S. Court of Appeals for the Second Circuit', 'Second Circuit', 'Second Circuit');

INSERT INTO judges (name, court_id, birthdate, appointed_date) VALUES
('John Doe', (SELECT id FROM courts WHERE name = 'U.S. District Court for the Southern District of New York'), '1970-01-01', '2010-01-01');

INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id, judge_id) VALUES
('1:21-cv-12345', 'Smith v. Johnson', 'Open', '2021-01-01',
 (SELECT id FROM courts WHERE name = 'U.S. District Court for the Southern District of New York'),
 (SELECT id FROM courts WHERE name = 'U.S. District Court for the Southern District of New York'),
 (SELECT id FROM judges WHERE name = 'John Doe'));


 -- Add more granular roles
 INSERT INTO roles (name, description) VALUES
 ('judge', 'Federal judge'),
 ('attorney', 'Practicing attorney'),
 ('clerk', 'Court clerk'),
 ('party', 'A party to a case'),
 ('admin', 'System administrator'),
 ('prosecutor', 'Government prosecutor'),
 ('public_defender', 'Public defender'),
 ('paralegal', 'Paralegal'),
 ('court_reporter', 'Court reporter'),
 ('probation_officer', 'Probation officer'),
 ('law_enforcement', 'Law enforcement officer'),
 ('expert_witness', 'Expert witness'),
 ('juror', 'Jury member'),
 ('mediator', 'Case mediator'),
 ('interpreter', 'Court interpreter')
ON CONFLICT (name) DO NOTHING;

 -- Add permissions
 INSERT INTO permissions (name, description) VALUES
 ('view_case', 'Can view case details'),
 ('edit_case', 'Can edit case details'),
 ('file_document', 'Can file documents'),
 ('schedule_hearing', 'Can schedule hearings'),
 ('view_sealed_documents', 'Can view sealed documents'),
 ('manage_users', 'Can manage user accounts'),
 ('generate_reports', 'Can generate system reports'),
 ('assign_cases', 'Can assign cases to judges'),
 ('manage_court_calendar', 'Can manage court calendar'),
 ('access_financial_records', 'Can access case financial records'),
 ('edit_docket', 'Can edit docket entries'),
 ('manage_jury', 'Can manage jury selection and instructions'),
 ('access_probation_records', 'Can access probation records'),
 ('edit_transcripts', 'Can edit court transcripts'),
 ('manage_evidence', 'Can manage case evidence')
ON CONFLICT (name) DO NOTHING;

 -- Assign permissions to roles (example assignments)
 INSERT INTO role_permissions (role_id, permission_id) VALUES
 ((SELECT id FROM roles WHERE name = 'judge'), (SELECT id FROM permissions WHERE name = 'view_case')),
 ((SELECT id FROM roles WHERE name = 'judge'), (SELECT id FROM permissions WHERE name = 'edit_case')),
 ((SELECT id FROM roles WHERE name = 'judge'), (SELECT id FROM permissions WHERE name = 'view_sealed_documents')),
 ((SELECT id FROM roles WHERE name = 'attorney'), (SELECT id FROM permissions WHERE name = 'view_case')),
 ((SELECT id FROM roles WHERE name = 'attorney'), (SELECT id FROM permissions WHERE name = 'file_document')),
 ((SELECT id FROM roles WHERE name = 'clerk'), (SELECT id FROM permissions WHERE name = 'edit_docket')),
 ((SELECT id FROM roles WHERE name = 'clerk'), (SELECT id FROM permissions WHERE name = 'schedule_hearing')),
 ((SELECT id FROM roles WHERE name = 'admin'), (SELECT id FROM permissions WHERE name = 'manage_users')),
 ((SELECT id FROM roles WHERE name = 'admin'), (SELECT id FROM permissions WHERE name = 'generate_reports'))
 ON CONFLICT DO NOTHING;


 ALTER TABLE cases ALTER COLUMN judge_id DROP NOT NULL;
