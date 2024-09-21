-- Part 1: Roles and Users
-- Roles
INSERT INTO roles (name, description)
VALUES
('judge', 'Federal judge'),
('attorney', 'Practicing attorney'),
('clerk', 'Court clerk'),
('admin', 'System administrator')
ON CONFLICT (name)
DO NOTHING;

-- Users (including judges and attorneys)
INSERT INTO users (username, password_hash, role_id)
VALUES
('jroberts', 'hash1', (SELECT id FROM roles WHERE name = 'judge')),
('rbader', 'hash2', (SELECT id FROM roles WHERE name = 'judge')),
('sbreyer', 'hash3', (SELECT id FROM roles WHERE name = 'judge')),
('nkatyal', 'hash4', (SELECT id FROM roles WHERE name = 'attorney')),
('tgoldstein', 'hash5', (SELECT id FROM roles WHERE name = 'attorney')),
('pclement', 'hash6', (SELECT id FROM roles WHERE name = 'attorney')),
('ckagan', 'hash7', (SELECT id FROM roles WHERE name = 'clerk')),
('msmith', 'hash8', (SELECT id FROM roles WHERE name = 'admin'))
ON CONFLICT (username)
DO NOTHING;

-- Part 2: Courts and Judges
-- Courts
INSERT INTO courts (name, district, circuit)
VALUES
('U.S. Supreme Court', 'Supreme Court', 'Supreme Court'),
('U.S. Court of Appeals for the Second Circuit', 'Second Circuit', 'Second Circuit'),
('U.S. District Court for the Southern District of New York', 'Southern District of New York', 'Second Circuit'),
('U.S. Court of Appeals for the Ninth Circuit', 'Ninth Circuit', 'Ninth Circuit'),
('U.S. District Court for the Northern District of California', 'Northern District of California', 'Ninth Circuit')
ON CONFLICT (name)
DO NOTHING;

-- Judges
INSERT INTO judges (name, court_id, birthdate, appointed_date)
SELECT j.name, c.id, j.birthdate::date, j.appointed_date::date
FROM (VALUES
    ('John G. Roberts Jr.', 'U.S. Supreme Court', '1955-01-27', '2005-09-29'),
    ('Sonia Sotomayor', 'U.S. Supreme Court', '1954-06-25', '2009-08-08'),
    ('Loretta A. Preska', 'U.S. District Court for the Southern District of New York', '1949-01-07', '1992-08-11'),
    ('Colleen McMahon', 'U.S. District Court for the Southern District of New York', '1951-10-06', '1998-10-21'),
    ('William H. Pauley III', 'U.S. District Court for the Southern District of New York', '1952-08-14', '1998-10-21'),
    ('Yvonne Gonzalez Rogers', 'U.S. District Court for the Northern District of California', '1965-07-25', '2011-11-21')
) AS j(name, court_name, birthdate, appointed_date)
JOIN courts c ON c.name = j.court_name
ON CONFLICT (name) DO NOTHING;

-- Part 3: Cases and Case Transfers
-- Cases
INSERT INTO cases (case_number, title, status, filed_date, closed_date, court_id, current_court_id, judge_id)
SELECT
    c.case_number, c.title, c.status,
    c.filed_date::date,
    c.closed_date::date,
    courts.id, current_courts.id, judges.id
FROM (VALUES
    ('1:20-cv-03010', 'United States v. Google LLC', 'Open', '2020-10-20', NULL,
     'U.S. District Court for the Southern District of New York',
     'U.S. District Court for the Southern District of New York', 'Loretta A. Preska'),
    ('3:20-cv-05640', 'Epic Games, Inc. v. Apple Inc.', 'Closed', '2020-08-13', '2021-09-10',
     'U.S. District Court for the Northern District of California',
     'U.S. District Court for the Northern District of California', 'Yvonne Gonzalez Rogers'),
    ('1:19-cv-03472', 'Donald J. Trump v. Deutsche Bank AG', 'Closed', '2019-04-29', '2020-12-21',
     'U.S. District Court for the Southern District of New York',
     'U.S. Supreme Court', 'John G. Roberts Jr.')
) AS c(case_number, title, status, filed_date, closed_date, court_name, current_court_name, judge_name)
JOIN courts ON courts.name = c.court_name
JOIN courts current_courts ON current_courts.name = c.current_court_name
JOIN judges ON judges.name = c.judge_name
ON CONFLICT (case_number, filed_date) DO NOTHING;

-- Case Transfers (for Trump v. Deutsche Bank case)
INSERT INTO case_transfers (case_id, from_court_id, to_court_id, transfer_date, transfer_reason)
SELECT
    cases.id,
    from_courts.id,
    to_courts.id,
    ct.transfer_date,
    ct.transfer_reason
FROM (VALUES
    ('1:19-cv-03472', 'U.S. District Court for the Southern District of New York',
     'U.S. Court of Appeals for the Second Circuit', '2019-08-23', 'Appeal filed'),
    ('1:19-cv-03472', 'U.S. Court of Appeals for the Second Circuit',
     'U.S. Supreme Court', '2019-12-13', 'Certiorari granted')
) AS ct(case_number, from_court_name, to_court_name, transfer_date, transfer_reason)
JOIN cases ON cases.case_number = ct.case_number
JOIN courts from_courts ON from_courts.name = ct.from_court_name
JOIN courts to_courts ON to_courts.name = ct.to_court_name
ON CONFLICT DO NOTHING;

-- Update current_court_id for transferred case
UPDATE cases
SET current_court_id = (SELECT id FROM courts WHERE name = 'U.S. Supreme Court')
WHERE case_number = '1:19-cv-03472';

-- Part 4: Documents, Hearings, Parties, and Dockets
-- Documents (for Trump v. Deutsche Bank case)
INSERT INTO documents (case_id, title, file_path, filed_date)
SELECT cases.id, d.title, d.file_path, d.filed_date
FROM (VALUES
    ('1:19-cv-03472', 'Complaint', '/documents/1:19-cv-03472/complaint.pdf', '2019-04-29'),
    ('1:19-cv-03472', 'Motion to Dismiss', '/documents/1:19-cv-03472/motion_to_dismiss.pdf', '2019-05-25'),
    ('1:19-cv-03472', 'Opposition to Motion to Dismiss', '/documents/1:19-cv-03472/opposition_motion_to_dismiss.pdf', '2019-06-10'),
    ('1:19-cv-03472', 'District Court Opinion', '/documents/1:19-cv-03472/district_court_opinion.pdf', '2019-08-16'),
    ('1:19-cv-03472', 'Notice of Appeal', '/documents/1:19-cv-03472/notice_of_appeal.pdf', '2019-08-23'),
    ('1:19-cv-03472', 'Appellate Brief', '/documents/1:19-cv-03472/appellate_brief.pdf', '2019-09-30'),
    ('1:19-cv-03472', 'Appellate Court Opinion', '/documents/1:19-cv-03472/appellate_court_opinion.pdf', '2019-12-03'),
    ('1:19-cv-03472', 'Petition for Writ of Certiorari', '/documents/1:19-cv-03472/cert_petition.pdf', '2019-12-06'),
    ('1:19-cv-03472', 'Supreme Court Brief', '/documents/1:19-cv-03472/supreme_court_brief.pdf', '2020-03-02'),
    ('1:19-cv-03472', 'Supreme Court Opinion', '/documents/1:19-cv-03472/supreme_court_opinion.pdf', '2020-07-09')
) AS d(case_number, title, file_path, filed_date)
JOIN cases ON cases.case_number = d.case_number
ON CONFLICT DO NOTHING;

-- Hearings (for Trump v. Deutsche Bank case)
INSERT INTO hearings (case_id, hearing_date, location, description)
SELECT cases.id, h.hearing_date, h.location, h.description
FROM (VALUES
    ('1:19-cv-03472', '2019-07-12 10:00:00', 'New York City Courthouse', 'Hearing on Motion to Dismiss'),
    ('1:19-cv-03472', '2019-11-15 10:00:00', 'New York City Courthouse', 'Oral Arguments at Second Circuit'),
    ('1:19-cv-03472', '2020-05-12 10:00:00', 'Supreme Court Building', 'Oral Arguments at Supreme Court')
) AS h(case_number, hearing_date, location, description)
JOIN cases ON cases.case_number = h.case_number
ON CONFLICT DO NOTHING;

-- Parties (for Trump v. Deutsche Bank case)
INSERT INTO parties (case_id, name, role, attorney_id)
SELECT cases.id, p.name, p.role, users.id
FROM (VALUES
    ('1:19-cv-03472', 'Donald J. Trump', 'Plaintiff', 'pclement'),
    ('1:19-cv-03472', 'Deutsche Bank AG', 'Defendant', 'tgoldstein'),
    ('1:19-cv-03472', 'Capital One Financial Corporation', 'Defendant', 'nkatyal')
) AS p(case_number, name, role, attorney_username)
JOIN cases ON cases.case_number = p.case_number
JOIN users ON users.username = p.attorney_username
ON CONFLICT DO NOTHING;

-- Dockets (for Trump v. Deutsche Bank case)
INSERT INTO dockets (case_id, entry_date, description, document_id)
SELECT
    cases.id,
    d.entry_date,
    d.description,
    documents.id
FROM (VALUES
    ('1:19-cv-03472', '2019-04-29', 'Complaint filed', 'Complaint'),
    ('1:19-cv-03472', '2019-05-25', 'Motion to Dismiss filed by Deutsche Bank AG', 'Motion to Dismiss'),
    ('1:19-cv-03472', '2019-06-10', 'Opposition to Motion to Dismiss filed', 'Opposition to Motion to Dismiss'),
    ('1:19-cv-03472', '2019-07-12', 'Hearing held on Motion to Dismiss', NULL),
    ('1:19-cv-03472', '2019-08-16', 'District Court Opinion issued', 'District Court Opinion'),
    ('1:19-cv-03472', '2019-08-23', 'Notice of Appeal filed', 'Notice of Appeal'),
    ('1:19-cv-03472', '2019-09-30', 'Appellate Brief filed', 'Appellate Brief'),
    ('1:19-cv-03472', '2019-11-15', 'Oral Arguments held at Second Circuit', NULL),
    ('1:19-cv-03472', '2019-12-03', 'Appellate Court Opinion issued', 'Appellate Court Opinion'),
    ('1:19-cv-03472', '2019-12-06', 'Petition for Writ of Certiorari filed', 'Petition for Writ of Certiorari'),
    ('1:19-cv-03472', '2019-12-13', 'Certiorari granted', NULL),
    ('1:19-cv-03472', '2020-03-02', 'Supreme Court Brief filed', 'Supreme Court Brief'),
    ('1:19-cv-03472', '2020-05-12', 'Oral Arguments held at Supreme Court', NULL),
    ('1:19-cv-03472', '2020-07-09', 'Supreme Court Opinion issued', 'Supreme Court Opinion'),
    ('1:19-cv-03472', '2020-12-21', 'Case closed', NULL)
) AS d(case_number, entry_date, description, document_title)
JOIN cases ON cases.case_number = d.case_number
LEFT JOIN documents ON documents.case_id = cases.id AND documents.title = d.document_title
ON CONFLICT DO NOTHING;

-- Part 5: Motions, Appeals, and Other Case-Related Data
-- Motions (for Trump v. Deutsche Bank case)
INSERT INTO motions (case_id, motion_type, filed_date, status, outcome)
SELECT cases.id, m.motion_type, m.filed_date, m.status, m.outcome
FROM (VALUES
    ('1:19-cv-03472', 'Motion to Dismiss', '2019-05-25', 'Decided', 'Denied'),
    ('1:19-cv-03472', 'Motion for Expedited Consideration', '2019-08-24', 'Decided', 'Granted'),
    ('1:19-cv-03472', 'Motion for Stay Pending Appeal', '2019-08-25', 'Decided', 'Granted')
) AS m(case_number, motion_type, filed_date, status, outcome)
JOIN cases ON cases.case_number = m.case_number
ON CONFLICT DO NOTHING;

-- Appeals (for Trump v. Deutsche Bank case)
INSERT INTO appeals (case_id, appeal_date, appellate_court, outcome)
SELECT cases.id, a.appeal_date, a.appellate_court, a.outcome
FROM (VALUES
    ('1:19-cv-03472', '2019-08-23', 'U.S. Court of Appeals for the Second Circuit', 'Affirmed'),
    ('1:19-cv-03472', '2019-12-06', 'U.S. Supreme Court', 'Vacated and Remanded')
) AS a(case_number, appeal_date, appellate_court, outcome)
JOIN cases ON cases.case_number = a.case_number
ON CONFLICT DO NOTHING;

-- Add some activity logs
INSERT INTO activity_logs (user_id, action, details)
SELECT users.id, al.action, al.details
FROM (VALUES
    ('pclement', 'Filed Document', 'Filed Complaint for case 1:19-cv-03472'),
    ('tgoldstein', 'Filed Motion', 'Filed Motion to Dismiss for case 1:19-cv-03472'),
    ('ckagan', 'Scheduled Hearing', 'Scheduled Oral Arguments for case 1:19-cv-03472 at Supreme Court')
) AS al(username, action, details)
JOIN users ON users.username = al.username
ON CONFLICT DO NOTHING;

-- Add some tasks
INSERT INTO tasks (case_id, assigned_to, task_description, due_date, status)
SELECT cases.id, users.id, t.task_description, t.due_date, t.status
FROM (VALUES
    ('1:19-cv-03472', 'ckagan', 'Prepare case materials for Supreme Court hearing', '2020-05-01', 'Completed')
) AS t(case_number, assignee_username, task_description, due_date, status)
JOIN cases ON cases.case_number = t.case_number
JOIN users ON users.username = t.assignee_username
ON CONFLICT DO NOTHING;

-- Part 6: Additional Case-Related Data
-- Add some fees and payments
INSERT INTO fees_payments (case_id, amount, payment_date, payment_method)
SELECT cases.id, fp.amount, fp.payment_date, fp.payment_method
FROM (VALUES
    ('1:19-cv-03472', 400.00, '2019-04-29', 'Credit Card'),
    ('1:19-cv-03472', 298.00, '2019-08-23', 'Electronic Funds Transfer'),
    ('1:19-cv-03472', 300.00, '2019-12-06', 'Credit Card')
) AS fp(case_number, amount, payment_date, payment_method)
JOIN cases ON cases.case_number = fp.case_number
ON CONFLICT DO NOTHING;

-- Add some scheduling orders
INSERT INTO scheduling_orders (case_id, order_date, description, deadline)
SELECT cases.id, so.order_date, so.description, so.deadline
FROM (VALUES
    ('1:19-cv-03472', '2019-05-01', 'Deadline for Motion to Dismiss', '2019-05-25'),
    ('1:19-cv-03472', '2019-05-01', 'Deadline for Opposition to Motion to Dismiss', '2019-06-10'),
    ('1:19-cv-03472', '2019-09-01', 'Deadline for Appellate Brief', '2019-09-30'),
    ('1:19-cv-03472', '2020-02-01', 'Deadline for Supreme Court Brief', '2020-03-02')
) AS so(case_number, order_date, description, deadline)
JOIN cases ON cases.case_number = so.case_number
ON CONFLICT DO NOTHING;

-- Add some discovery requests
INSERT INTO discovery (case_id, request_type, request_date, response_date, status)
SELECT cases.id, d.request_type, d.request_date, d.response_date, d.status
FROM (VALUES
    ('1:19-cv-03472', 'Request for Production of Documents', '2019-05-15', '2019-06-14', 'Completed'),
    ('1:19-cv-03472', 'Interrogatories', '2019-05-15', '2019-06-14', 'Completed'),
    ('1:19-cv-03472', 'Request for Admissions', '2019-05-20', '2019-06-19', 'Completed')
) AS d(case_number, request_type, request_date, response_date, status)
JOIN cases ON cases.case_number = d.case_number
ON CONFLICT DO NOTHING;

-- Add some pleadings
INSERT INTO pleadings (case_id, pleading_type, filed_date, status)
SELECT cases.id, p.pleading_type, p.filed_date, p.status
FROM (VALUES
    ('1:19-cv-03472', 'Complaint', '2019-04-29', 'Filed'),
    ('1:19-cv-03472', 'Answer', '2019-06-25', 'Filed')
) AS p(case_number, pleading_type, filed_date, status)
JOIN cases ON cases.case_number = p.case_number
ON CONFLICT DO NOTHING;

-- Add some pretrial conferences
INSERT INTO pretrial_conferences (case_id, conference_date, location, description)
SELECT cases.id, pc.conference_date, pc.location, pc.description
FROM (VALUES
    ('1:19-cv-03472', '2019-06-01 10:00:00', 'New York City Courthouse', 'Initial Pretrial Conference'),
    ('1:19-cv-03472', '2019-07-01 10:00:00', 'New York City Courthouse', 'Final Pretrial Conference')
) AS pc(case_number, conference_date, location, description)
JOIN cases ON cases.case_number = pc.case_number
ON CONFLICT DO NOTHING;

-- Add judgment
INSERT INTO judgments (case_id, judgment_date, description)
SELECT cases.id, j.judgment_date, j.description
FROM (VALUES
    ('1:19-cv-03472', '2020-07-09', 'Supreme Court vacated the judgment of the Court of Appeals and remanded the case for further proceedings consistent with its opinion.')
) AS j(case_number, judgment_date, description)
JOIN cases ON cases.case_number = j.case_number
ON CONFLICT DO NOTHING;

-- Part 7: Case History and Document Versions
-- Add some case history entries
INSERT INTO case_history (case_id, change_date, changed_by, change_description)
SELECT cases.id, ch.change_date, users.id, ch.change_description
FROM (VALUES
    ('1:19-cv-03472', '2019-04-29', 'ckagan', 'Case filed'),
    ('1:19-cv-03472', '2019-08-23', 'ckagan', 'Case appealed to Second Circuit'),
    ('1:19-cv-03472', '2019-12-13', 'ckagan', 'Case accepted by Supreme Court'),
    ('1:19-cv-03472', '2020-07-09', 'ckagan', 'Supreme Court decision issued'),
    ('1:19-cv-03472', '2020-12-21', 'ckagan', 'Case closed')
) AS ch(case_number, change_date, changed_by_username, change_description)
JOIN cases ON cases.case_number = ch.case_number
JOIN users ON users.username = ch.changed_by_username
ON CONFLICT DO NOTHING;

-- Add some document versions
INSERT INTO document_versions (document_id, version_number, file_path)
SELECT documents.id, dv.version_number, dv.file_path
FROM (VALUES
    ('1:19-cv-03472', 'Complaint', 1, '/documents/1:19-cv-03472/complaint_v1.pdf'),
    ('1:19-cv-03472', 'Complaint', 2, '/documents/1:19-cv-03472/complaint_v2.pdf')
) AS dv(case_number, document_title, version_number, file_path)
JOIN cases ON cases.case_number = dv.case_number
JOIN documents ON documents.case_id = cases.id AND documents.title = dv.document_title
ON CONFLICT DO NOTHING;

-- Part 8: User Actions and Analytics
-- Add some user actions
INSERT INTO user_actions (user_id, action_type, details)
SELECT users.id, ua.action_type, ua.details
FROM (VALUES
    ('pclement', 'View Case', 'Viewed case 1:19-cv-03472'),
    ('tgoldstein', 'File Document', 'Filed Motion to Dismiss for case 1:19-cv-03472'),
    ('ckagan', 'Update Case', 'Updated status of case 1:19-cv-03472')
) AS ua(username, action_type, details)
JOIN users ON users.username = ua.username
ON CONFLICT DO NOTHING;

-- Add some user frequent actions
INSERT INTO user_frequent_actions (user_id, action_type, frequency, last_used)
SELECT users.id, ufa.action_type, ufa.frequency, ufa.last_used
FROM (VALUES
    ('pclement', 'View Case', 50, '2020-12-21 15:30:00'),
    ('tgoldstein', 'File Document', 30, '2020-12-20 11:45:00'),
    ('ckagan', 'Update Case', 100, '2020-12-21 16:20:00')
) AS ufa(username, action_type, frequency, last_used)
JOIN users ON users.username = ufa.username
ON CONFLICT DO NOTHING;

-- Add some user last actions
INSERT INTO user_last_actions (user_id, action_type, last_used)
SELECT users.id, ula.action_type, ula.last_used
FROM (VALUES
    ('pclement', 'View Document', '2020-12-21 17:15:00'),
    ('tgoldstein', 'Search Cases', '2020-12-21 16:30:00'),
    ('ckagan', 'Generate Report', '2020-12-21 18:00:00')
) AS ula(username, action_type, last_used)
JOIN users ON users.username = ula.username
ON CONFLICT DO NOTHING;

-- Part 9: Chat and Communication
-- Add a chat room for the case
INSERT INTO chat_rooms (case_id, name)
SELECT cases.id, cr.name
FROM (VALUES
    ('1:19-cv-03472', 'Trump v. Deutsche Bank Discussion')
) AS cr(case_number, name)
JOIN cases ON cases.case_number = cr.case_number
ON CONFLICT DO NOTHING;

-- Add some messages to the chat room
INSERT INTO messages (chat_room_id, user_id, content)
SELECT chat_rooms.id, users.id, m.content
FROM (VALUES
    ('Trump v. Deutsche Bank Discussion', 'pclement', 'Has everyone reviewed the latest Supreme Court brief?'),
    ('Trump v. Deutsche Bank Discussion', 'tgoldstein', 'Yes, I've gone through it. Let's discuss the key points.'),
    ('Trump v. Deutsche Bank Discussion', 'ckagan', 'I've uploaded the brief to the case documents. You can find it there.')
) AS m(chat_room_name, username, content)
JOIN chat_rooms ON chat_rooms.name = m.chat_room_name
JOIN users ON users.username = m.username
ON CONFLICT DO NOTHING;

-- Add chat participants
INSERT INTO chat_participants (chat_room_id, user_id)
SELECT chat_rooms.id, users.id
FROM (VALUES
    ('Trump v. Deutsche Bank Discussion', 'pclement'),
    ('Trump v. Deutsche Bank Discussion', 'tgoldstein'),
    ('Trump v. Deutsche Bank Discussion', 'ckagan')
) AS cp(chat_room_name, username)
JOIN chat_rooms ON chat_rooms.name = cp.chat_room_name
JOIN users ON users.username = cp.username
ON CONFLICT DO NOTHING;

-- Add some chat notifications
INSERT INTO chat_notifications (user_id, chat_room_id, message_id, is_read)
SELECT
    users.id,
    chat_rooms.id,
    messages.id,
    cn.is_read
FROM (VALUES
    ('pclement', 'Trump v. Deutsche Bank Discussion', 'I've uploaded the brief to the case documents. You can find it there.', false)
) AS cn(username, chat_room_name, message_content, is_read)
JOIN users ON users.username = cn.username
JOIN chat_rooms ON chat_rooms.name = cn.chat_room_name
JOIN messages ON messages.content = cn.message_content
ON CONFLICT DO NOTHING;

-- Add some user presence data
INSERT INTO user_presence (user_id, status)
SELECT users.id, up.status
FROM (VALUES
    ('pclement', 'online'),
    ('tgoldstein', 'away'),
    ('ckagan', 'offline')
) AS up(username, status)
JOIN users ON users.username = up.username
ON CONFLICT DO NOTHING;

-- Refresh the materialized view
REFRESH MATERIALIZED VIEW cases_summary;
