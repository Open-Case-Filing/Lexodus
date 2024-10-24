-- Part 1: Roles and Users
INSERT INTO roles (name, description, permissions) VALUES
('judge', 'Federal judge', '{"can_view_all_cases": true, "can_edit_cases": true, "can_issue_orders": true}'::jsonb),
('attorney', 'Practicing attorney', '{"can_file_cases": true, "can_view_own_cases": true, "can_file_motions": true}'::jsonb),
('clerk', 'Court clerk', '{"can_manage_docket": true, "can_view_all": true, "can_schedule": true}'::jsonb),
('admin', 'System administrator', '{"can_manage_all": true, "can_manage_users": true}'::jsonb)
ON CONFLICT (name) DO NOTHING;

INSERT INTO users (
    username,
    email,
    password_hash,
    full_name,
    role_id,
    biography,
    two_factor_enabled,
    last_login_at
) VALUES
('jroberts', 'jroberts@courts.gov', 'hash1', 'John G. Roberts Jr.',
    (SELECT id FROM roles WHERE name = 'judge'),
    'Chief Justice of the United States Supreme Court', false, NOW()),
('rbader', 'rbader@courts.gov', 'hash2', 'Ruth Bader Ginsburg',
    (SELECT id FROM roles WHERE name = 'judge'),
    'Associate Justice of the Supreme Court', false, NOW()),
('sbreyer', 'sbreyer@courts.gov', 'hash3', 'Stephen Breyer',
    (SELECT id FROM roles WHERE name = 'judge'),
    'Associate Justice of the Supreme Court', false, NOW()),
('nkatyal', 'nkatyal@law.firm', 'hash4', 'Neal Katyal',
    (SELECT id FROM roles WHERE name = 'attorney'),
    'Partner at Law Firm', false, NOW()),
('tgoldstein', 'tgoldstein@law.firm', 'hash5', 'Tom Goldstein',
    (SELECT id FROM roles WHERE name = 'attorney'),
    'Supreme Court Advocate', false, NOW()),
('pclement', 'pclement@law.firm', 'hash6', 'Paul Clement',
    (SELECT id FROM roles WHERE name = 'attorney'),
    'Former Solicitor General', false, NOW()),
('ckagan', 'ckagan@courts.gov', 'hash7', 'Clerk Kagan',
    (SELECT id FROM roles WHERE name = 'clerk'),
    'Supreme Court Clerk', false, NOW()),
('msmith', 'msmith@courts.gov', 'hash8', 'Admin Smith',
    (SELECT id FROM roles WHERE name = 'admin'),
    'System Administrator', false, NOW())
ON CONFLICT (username) DO NOTHING;

-- Part 2: Courts
INSERT INTO courts (
    name,
    court_type,
    district,
    circuit,
    jurisdiction,
    physical_address,
    filing_hours,
    is_active
) VALUES
('U.S. Supreme Court', 'SUPREME', 'Supreme Court', 'Supreme Court',
    ARRAY['FEDERAL', 'APPELLATE']::text[],
    '1 First St NE, Washington, DC 20543',
    '{"weekday": {"open": "9:00", "close": "17:00"}}'::jsonb,
    true),
('U.S. Court of Appeals for the Second Circuit', 'APPELLATE', 'Second Circuit', 'Second Circuit',
    ARRAY['FEDERAL', 'APPELLATE']::text[],
    '40 Foley Square, New York, NY 10007',
    '{"weekday": {"open": "8:30", "close": "17:00"}}'::jsonb,
    true),
('U.S. District Court for the Southern District of New York', 'DISTRICT', 'Southern District of New York', 'Second Circuit',
    ARRAY['FEDERAL', 'TRIAL']::text[],
    '500 Pearl St, New York, NY 10007',
    '{"weekday": {"open": "8:30", "close": "17:00"}}'::jsonb,
    true),
('U.S. Court of Appeals for the Ninth Circuit', 'APPELLATE', 'Ninth Circuit', 'Ninth Circuit',
    ARRAY['FEDERAL', 'APPELLATE']::text[],
    '95 7th St, San Francisco, CA 94103',
    '{"weekday": {"open": "8:30", "close": "17:00"}}'::jsonb,
    true),
('U.S. District Court for the Northern District of California', 'DISTRICT', 'Northern District of California', 'Ninth Circuit',
    ARRAY['FEDERAL', 'TRIAL']::text[],
    '450 Golden Gate Ave, San Francisco, CA 94102',
    '{"weekday": {"open": "8:30", "close": "17:00"}}'::jsonb,
    true)
ON CONFLICT (name, district) DO NOTHING;

-- Part 3: Judicial Officers
INSERT INTO judicial_officers (
    user_id,
    court_id,
    title,
    status,
    appointment_date,
    specialties,
    is_chief_judge
)
SELECT
    u.id,
    c.id,
    CASE
        WHEN u.full_name = 'John G. Roberts Jr.' THEN 'Chief Justice'
        ELSE 'Associate Justice'
    END,
    'ACTIVE',
    CASE
        WHEN u.full_name = 'John G. Roberts Jr.' THEN '2005-09-29'::date
        WHEN u.full_name = 'Ruth Bader Ginsburg' THEN '1993-08-10'::date
        WHEN u.full_name = 'Stephen Breyer' THEN '1994-08-03'::date
    END,
    ARRAY['Constitutional Law', 'Administrative Law']::text[],
    u.full_name = 'John G. Roberts Jr.'
FROM users u
JOIN courts c ON c.name = 'U.S. Supreme Court'
WHERE u.username IN ('jroberts', 'rbader', 'sbreyer')
ON CONFLICT (user_id, court_id) DO NOTHING;

-- Part 4: Document Types
INSERT INTO document_types (
    category,
    name,
    description,
    requires_fee,
    requires_service
) VALUES
('PLEADING', 'Complaint', 'Initial pleading that starts a civil action', true, true),
('MOTION', 'Motion to Dismiss', 'Motion to dismiss the case', false, true),
('ORDER', 'Court Order', 'Order issued by the court', false, false),
('NOTICE', 'Notice of Appeal', 'Notice of intent to appeal', true, true)
ON CONFLICT (category, name) DO NOTHING;

-- Part 5: Cases
INSERT INTO cases (
    case_number,
    title,
    case_type,
    nature_of_suit,
    status,
    filed_date,
    court_id,
    assigned_judge_id,
    security_level,
    created_by,
    filing_type
)
SELECT
    c.case_number,
    c.title,
    'CIVIL',
    'Federal Question',
    c.status,
    c.filed_date::date,
    courts.id,
    jo.id,
    'PUBLIC',
    u.id,
    'ELECTRONIC'
FROM (VALUES
    ('1:20-cv-03010', 'United States v. Google LLC', 'OPEN', '2020-10-20',
     'U.S. District Court for the Southern District of New York', 'John G. Roberts Jr.'),
    ('3:20-cv-05640', 'Epic Games, Inc. v. Apple Inc.', 'CLOSED', '2020-08-13',
     'U.S. District Court for the Northern District of California', 'Ruth Bader Ginsburg')
) AS c(case_number, title, status, filed_date, court_name, judge_name)
JOIN courts ON courts.name = c.court_name
JOIN users u ON u.username = 'ckagan'  -- Clerk creating the cases
JOIN judicial_officers jo ON jo.user_id = (
    SELECT id FROM users WHERE full_name = c.judge_name
)
ON CONFLICT DO NOTHING;



-- Part 6: Documents
INSERT INTO documents (
    case_id,
    document_type_id,
    title,
    description,
    storage_path,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic
)
SELECT
    c.id,
    dt.id,
    d.title,
    d.description,
    d.storage_path,
    1024,  -- 1KB dummy file size
    'PUBLIC',
    u.id,
    c.filed_date,
    true
FROM cases c
CROSS JOIN (VALUES
    ('Complaint', 'Initial complaint filing', '/documents/complaint.pdf', 'PLEADING'),
    ('Motion to Dismiss', 'Motion to dismiss case', '/documents/motion_dismiss.pdf', 'MOTION'),
    ('Court Order', 'Initial scheduling order', '/documents/order.pdf', 'ORDER')
) AS d(title, description, storage_path, doc_type)
JOIN document_types dt ON dt.name = d.doc_type
JOIN users u ON u.username = 'pclement'  -- Attorney filing documents
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 7: Calendar Events
INSERT INTO calendar_events (
    title,
    description,
    event_type,
    start_time,
    end_time,
    location_type,
    location_details,
    case_id,
    organizer_id,
    participants,
    created_by
)
SELECT
    'Initial Conference',
    'Initial scheduling conference',
    'HEARING',
    c.filed_date + interval '30 days',
    c.filed_date + interval '30 days' + interval '1 hour',
    'COURTROOM',
    '{"room": "Courtroom 1", "floor": "3rd Floor"}'::jsonb,
    c.id,
    u.id,
    '[{"role": "JUDGE", "user_id": 1}, {"role": "ATTORNEY", "user_id": 2}]'::jsonb,
    u.id
FROM cases c
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 8: Tasks
INSERT INTO tasks (
    title,
    description,
    task_type,
    priority,
    case_id,
    assigned_to,
    assigned_by,
    due_date,
    status
)
SELECT
    'Review Initial Filing',
    'Review complaint for compliance with local rules',
    'DOCUMENT_REVIEW',
    'HIGH',
    c.id,
    u2.id,  -- Assigned to clerk
    u1.id,  -- Assigned by judge
    c.filed_date + interval '7 days',
    'PENDING'
FROM cases c
JOIN users u1 ON u1.username = 'jroberts'
JOIN users u2 ON u2.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 9: Workflow Templates and Instances
INSERT INTO workflow_templates (
    name,
    description,
    category,
    steps,
    transitions,
    created_by
)
SELECT
    'Civil Case Initial Review',
    'Workflow for initial civil case review and processing',
    'CASE_PROCESSING',
    '[
        {"id": "INITIAL_REVIEW", "name": "Initial Review", "role": "CLERK"},
        {"id": "JUDGE_REVIEW", "name": "Judicial Review", "role": "JUDGE"},
        {"id": "SCHEDULING", "name": "Scheduling", "role": "CLERK"}
    ]'::jsonb,
    '[
        {"from": "INITIAL_REVIEW", "to": "JUDGE_REVIEW"},
        {"from": "JUDGE_REVIEW", "to": "SCHEDULING"}
    ]'::jsonb,
    u.id
FROM users u
WHERE u.username = 'msmith'
ON CONFLICT DO NOTHING;

-- Create workflow instances
INSERT INTO workflows (
    template_id,
    case_id,
    title,
    description,
    current_step,
    status,
    start_date,
    created_by
)
SELECT
    wt.id,
    c.id,
    'Initial Case Review - ' || c.case_number,
    'Initial review workflow for ' || c.title,
    'INITIAL_REVIEW',
    'ACTIVE',
    NOW(),
    u.id
FROM workflow_templates wt
JOIN cases c ON c.case_number = '1:20-cv-03010'
JOIN users u ON u.username = 'ckagan'
WHERE wt.name = 'Civil Case Initial Review'
ON CONFLICT DO NOTHING;

-- Part 10: Chat Rooms and Messages
INSERT INTO chat_rooms (
    name,
    room_type,
    description,
    case_id,
    created_by
)
SELECT
    'Case Discussion - ' || c.case_number,
    'CASE',
    'Discussion room for case ' || c.title,
    c.id,
    u.id
FROM cases c
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Add chat participants
INSERT INTO chat_participants (
    chat_room_id,
    user_id,
    role,
    invited_by
)
SELECT
    cr.id,
    u.id,
    'MEMBER',
    cr.created_by
FROM chat_rooms cr
CROSS JOIN users u
WHERE u.username IN ('jroberts', 'pclement', 'ckagan')
AND cr.room_type = 'CASE'
ON CONFLICT DO NOTHING;

-- Add some initial messages
INSERT INTO messages (
    chat_room_id,
    sender_id,
    content,
    content_type
)
SELECT
    cr.id,
    u.id,
    'Initial case review completed. Ready for judicial review.',
    'TEXT'
FROM chat_rooms cr
JOIN users u ON u.username = 'ckagan'
WHERE cr.room_type = 'CASE'
ON CONFLICT DO NOTHING;

-- Part 11: Security Classifications
INSERT INTO security_classifications (
    name,
    description,
    access_level,
    requires_clearance
) VALUES
('PUBLIC', 'Public accessible records', 1, false),
('SEALED', 'Sealed court records', 2, true),
('CLASSIFIED', 'Classified national security information', 3, true)
ON CONFLICT DO NOTHING;

-- Grant clearances to users
INSERT INTO user_security_clearances (
    user_id,
    classification_id,
    granted_by,
    granted_at
)
SELECT
    u.id,
    sc.id,
    admin.id,
    NOW()
FROM users u
CROSS JOIN security_classifications sc
JOIN users admin ON admin.username = 'msmith'
WHERE u.username IN ('jroberts', 'rbader', 'sbreyer')
AND sc.name IN ('SEALED', 'CLASSIFIED')
ON CONFLICT DO NOTHING;


-- Part 12: Event Types (needed before case_events)
INSERT INTO event_types (
    name,
    category,
    description,
    requires_document,
    requires_judicial_review,
    auto_notification
) VALUES
('FILING', 'DOCUMENT', 'Document filing event', true, false, true),
('HEARING', 'CALENDAR', 'Court hearing', false, true, true),
('DISCOVERY', 'DOCUMENT', 'Discovery related event', true, false, true),
('ORDER', 'JUDICIAL', 'Court order', true, true, true),
('MOTION', 'DOCUMENT', 'Motion filing', true, true, true),
('NOTICE', 'DOCUMENT', 'Notice filing', true, false, true),
('STATUS_CHANGE', 'CASE', 'Case status change', false, true, true)
ON CONFLICT DO NOTHING;

-- Part 13: Case Events
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    c.filed_date + interval '45 days',
    d.title,
    d.description,
    d.public_entry,
    u_filing.id,
    u_clerk.id,
    'ACTIVE'
FROM cases c
CROSS JOIN (VALUES
    ('Discovery Request', 'Request for Production of Documents', 'Discovery request served on opposing party'),
    ('Discovery Request', 'First Set of Interrogatories', 'Interrogatories served on opposing party'),
    ('Discovery Request', 'Request for Admissions', 'Request for admissions served on opposing party')
) as d(title, description, public_entry)
JOIN event_types et ON et.name = 'DISCOVERY'
JOIN users u_filing ON u_filing.username = 'pclement'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 14: Case Deadlines
INSERT INTO case_deadlines (
    case_id,
    case_filed_date,
    title,
    deadline_type,
    due_date,
    priority,
    status,
    reminder_days,
    created_by
)
SELECT
    c.id,
    c.filed_date,
    d.title,
    d.deadline_type,
    c.filed_date + d.days_offset,
    'NORMAL',
    'PENDING',
    ARRAY[1, 7, 14],
    u.id
FROM cases c
CROSS JOIN (VALUES
    ('Answer Due', 'RESPONSE', 21),
    ('Initial Disclosures', 'DISCOVERY', 30),
    ('Discovery Cutoff', 'DISCOVERY', 120),
    ('Dispositive Motions Deadline', 'MOTION', 150)
) as d(title, deadline_type, days_offset)
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 15: Motion Types
INSERT INTO motion_types (
    name,
    category,
    description,
    requires_hearing,
    requires_response,
    standard_response_days,
    auto_scheduling_enabled,
    created_at,
    updated_at
) VALUES
('Motion to Dismiss'::text, 'DISPOSITIVE'::text, 'Motion to dismiss the case'::text, true, true, 21, false, NOW(), NOW()),
('Motion for Summary Judgment'::text, 'DISPOSITIVE'::text, 'Request for judgment based on undisputed facts'::text, true, true, 21, false, NOW(), NOW()),
('Motion for Extension of Time'::text, 'PROCEDURAL'::text, 'Request for deadline extension'::text, false, true, 14, false, NOW(), NOW()),
('Motion to Compel'::text, 'DISCOVERY'::text, 'Motion to compel discovery responses'::text, false, true, 14, false, NOW(), NOW())
ON CONFLICT (name) DO NOTHING;
-- Part 16: Case Status History
INSERT INTO case_status_history (
    case_id,
    case_filed_date,
    old_status,
    new_status,
    changed_by,
    change_date,
    notes
)
SELECT
    c.id,
    c.filed_date,
    'DRAFT',
    'OPEN',
    u.id,
    c.filed_date,
    'Case initially filed'
FROM cases c
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 17: Document Versions
INSERT INTO document_versions (
    document_id,
    version_number,
    storage_path,
    checksum,
    file_size,
    changed_by,
    change_reason,
    created_at
)
SELECT
    d.id,
    1,
    d.storage_path || '.v1',
    md5(random()::text),
    1024,
    u.id,
    'Initial version',
    NOW()
FROM documents d
JOIN users u ON u.username = 'pclement'
WHERE d.title = 'Complaint'
ON CONFLICT DO NOTHING;

-- Part 18: Deadline Assignments
INSERT INTO deadline_assignments (
    deadline_id,
    assigned_user_id,
    assigned_by,
    assigned_date,
    status,
    notes
)
SELECT
    cd.id,
    u_assignee.id,
    u_assigner.id,
    NOW(),
    'ACTIVE',
    'Please handle this deadline'
FROM case_deadlines cd
JOIN cases c ON c.id = cd.case_id
JOIN users u_assignee ON u_assignee.username = 'ckagan'
JOIN users u_assigner ON u_assigner.username = 'jroberts'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 19: Electronic Signatures
INSERT INTO electronic_signatures (
    document_id,
    document_version_id,
    signer_id,
    signature_type,
    signature_date,
    signature_location,
    signature_capacity,
    verification_token,
    verification_method,
    ip_address
)
SELECT
    d.id,
    dv.id,
    u.id,
    'S-SIGNATURE',
    NOW(),
    'Washington, DC',
    'ATTORNEY',
    md5(random()::text),
    'EMAIL',
    '127.0.0.1'::inet
FROM documents d
JOIN document_versions dv ON dv.document_id = d.id
JOIN users u ON u.username = 'pclement'
WHERE d.title = 'Complaint'
ON CONFLICT DO NOTHING;

-- Part 20: Access Control Lists
INSERT INTO access_control_lists (
    resource_type,
    resource_id,
    principal_type,
    principal_id,
    permissions,
    granted_by
)
SELECT
    'CASE',
    c.id,
    'ROLE',
    r.id,
    '{"permissions": ["VIEW", "EDIT", "DELETE"]}'::jsonb,
    u.id
FROM cases c
CROSS JOIN roles r
JOIN users u ON u.username = 'msmith'
WHERE c.case_number = '1:20-cv-03010'
AND r.name IN ('judge', 'clerk')
ON CONFLICT DO NOTHING;


-- Part 21: Activity Logs for the case
INSERT INTO activity_logs (
    user_id,
    role_id,
    action_type,
    action_category,
    action_description,
    entity_type,
    entity_id,
    entity_name,
    case_id,
    ip_address,
    status,
    metadata
)
SELECT
    u.id,
    u.role_id,
    a.action_type,
    a.category,
    a.description,
    'CASE',
    c.id::text,
    c.title,
    c.id,
    '127.0.0.1'::inet,
    'SUCCESS',
    jsonb_build_object(
        'browser', 'Chrome',
        'platform', 'Windows',
        'details', a.details
    )
FROM cases c
CROSS JOIN (VALUES
    ('CASE_CREATED', 'FILING', 'Case initially filed', 'Initial case filing'),
    ('DOCUMENT_FILED', 'DOCUMENT', 'Complaint filed', 'Initial complaint'),
    ('JUDGE_ASSIGNED', 'ASSIGNMENT', 'Case assigned to judge', 'Judicial assignment'),
    ('HEARING_SCHEDULED', 'CALENDAR', 'Initial conference scheduled', 'Scheduling'),
    ('SERVICE_COMPLETED', 'PROCESS', 'Service completed on defendant', 'Process service'),
    ('MOTION_FILED', 'MOTION', 'Motion to dismiss filed', 'Motion filing')
) as a(action_type, category, description, details)
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 22: Case Related Documents
-- Part 22: Case Related Documents
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,  -- Added required field
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title,
    d.description,
    '/documents/' || c.case_number || '/' || lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || c.case_number)::text,  -- Generate a deterministic checksum
    d.file_size,
    'PUBLIC',
    u.id,
    c.filed_date + d.days_offset,
    true
FROM cases c
CROSS JOIN (VALUES
    ('Civil Cover Sheet', 'Initial civil cover sheet', 0, 500),
    ('Summons', 'Summons to be served on defendant', 0, 200),
    ('Corporate Disclosure Statement', 'Statement of corporate interests', 1, 300),
    ('Notice of Appearance', 'Attorney notice of appearance', 2, 150),
    ('Certificate of Service', 'Proof of service on defendant', 5, 250),
    ('Initial Disclosures', 'Rule 26(a) initial disclosures', 14, 1500)
) as d(title, description, days_offset, file_size)
JOIN document_types dt ON dt.category = 'PLEADING'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 23: Docket Entries for Case Timeline
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    c.filed_date + d.days_offset,
    d.title,
    d.description,
    d.public_entry,
    u_filing.id,
    u_clerk.id,
    'ACTIVE'
FROM cases c
CROSS JOIN (VALUES
    (0, 'Case Filing', 'Case filed', 'Case filed. Initial filing fee paid.'),
    (0, 'Cover Sheet Filing', 'Civil cover sheet filed', 'Civil cover sheet filed.'),
    (1, 'Judge Assignment', 'Case assigned to judge', 'Case assigned to Judge Roberts.'),
    (2, 'Summons Issuance', 'Summons issued', 'Summons issued to defendant.'),
    (5, 'Attorney Appearance', 'Notice of appearance filed', 'Notice of appearance filed for plaintiff.'),
    (7, 'Service Filing', 'Certificate of service filed', 'Certificate of service filed.'),
    (14, 'Scheduling Notice', 'Initial conference notice', 'Initial conference notice issued.'),
    (21, 'Motion Filing', 'Motion to dismiss filed', 'Defendant''s motion to dismiss filed.'),
    (30, 'Conference Held', 'Initial conference held', 'Initial pretrial conference held.')
) as d(days_offset, title, description, public_entry)
JOIN event_types et ON et.name = 'FILING'  -- Using generic FILING type, you might want to map specific event types
JOIN users u_filing ON u_filing.username = 'pclement'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;


-- Part 24: Workflow History
INSERT INTO workflow_history (
    workflow_id,
    step_from,
    step_to,
    action_taken,
    action_result,
    performed_by,
    comments
)
SELECT
    w.id,
    wh.step_from,
    wh.step_to,
    wh.action,
    'COMPLETED',
    u.id,
    wh.comments
FROM workflows w
JOIN cases c ON c.id = w.case_id
CROSS JOIN (VALUES
    (NULL, 'INITIAL_REVIEW', 'Initial review started', 'Beginning case review'),
    ('INITIAL_REVIEW', 'JUDGE_REVIEW', 'Initial review completed', 'Ready for judicial review'),
    ('JUDGE_REVIEW', 'SCHEDULING', 'Judicial review completed', 'Approved for scheduling')
) as wh(step_from, step_to, action, comments)
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;




-- Part 24: Case Parties
INSERT INTO case_parties (
    case_id,
    case_filed_date,
    party_type_id,
    name,
    is_lead,
    address,
    is_pro_se,
    company_details
)
SELECT
    c.id,
    c.filed_date,
    pt.id,
    p.name,
    p.is_lead,
    jsonb_build_object(
        'street', p.street,
        'city', p.city,
        'state', p.state,
        'zip', p.zip
    ),
    false,
    CASE
        WHEN p.company_type IS NOT NULL
        THEN jsonb_build_object(
            'type', p.company_type,
            'registration', p.registration,
            'jurisdiction', p.jurisdiction
        )
        ELSE NULL
    END
FROM cases c
CROSS JOIN (VALUES
    ('United States of America', true, '950 Pennsylvania Avenue NW', 'Washington', 'DC', '20530',
     'Government', 'USA-GOV', 'Federal'),
    ('Google LLC', true, '1600 Amphitheatre Parkway', 'Mountain View', 'CA', '94043',
     'Corporation', 'DE-12345', 'Delaware'),
    ('State of Texas', false, '300 W. 15th Street', 'Austin', 'TX', '78701',
     'Government', 'TX-GOV', 'State'),
    ('State of New York', false, '28 Liberty Street', 'New York', 'NY', '10005',
     'Government', 'NY-GOV', 'State')
) as p(name, is_lead, street, city, state, zip, company_type, registration, jurisdiction)
JOIN party_types pt ON pt.name = CASE
    WHEN p.name = 'United States of America' THEN 'PLAINTIFF'
    ELSE 'DEFENDANT'
    END
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 25: Attorney Representations
INSERT INTO party_attorney_representations (
    case_party_id,
    attorney_id,
    representation_type,
    representation_start_date,
    status,
    created_by
)
SELECT
    cp.id,
    u.id,
    'LEAD',
    c.filed_date,
    'ACTIVE',
    admin.id
FROM cases c
JOIN case_parties cp ON cp.case_id = c.id
JOIN users u ON u.username IN ('pclement', 'nkatyal', 'tgoldstein')
JOIN users admin ON admin.username = 'msmith'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 26: Case Activity Records for Party Actions
INSERT INTO activity_logs (
    user_id,
    role_id,
    action_type,
    action_category,
    action_description,
    entity_type,
    entity_id,
    entity_name,
    case_id,
    ip_address,
    status,
    metadata
)
SELECT
    u.id,
    u.role_id,
    'PARTY_ADDED',
    'CASE_PARTY',
    'Added party: ' || cp.name,
    'PARTY',
    cp.id::text,
    cp.name,
    c.id,
    '127.0.0.1'::inet,
    'SUCCESS',
    jsonb_build_object(
        'party_type', pt.name,
        'is_lead', cp.is_lead,
        'added_by', u.username
    )
FROM cases c
JOIN case_parties cp ON cp.case_id = c.id
JOIN party_types pt ON pt.id = cp.party_type_id
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 27: Party Related Documents
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title || ' - ' || cp.name,
    d.description,
    '/documents/' || c.case_number || '/parties/' || lower(replace(cp.name, ' ', '_')) || '/' ||
        lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || cp.name || c.case_number)::text,
    d.file_size,
    'PUBLIC',
    u.id,
    c.filed_date + d.days_offset,
    true
FROM cases c
JOIN case_parties cp ON cp.case_id = c.id
CROSS JOIN (VALUES
    ('Notice of Appearance', 'Attorney appearance for party', 1, 250),
    ('Corporate Disclosure', 'Corporate disclosure statement', 2, 500),
    ('Pro Hac Vice Motion', 'Motion for pro hac vice admission', 3, 300)
) as d(title, description, days_offset, file_size)
JOIN document_types dt ON dt.category = 'PLEADING'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
AND cp.is_lead = true
ON CONFLICT DO NOTHING;


-- Part 28: Discovery Events and Tracking
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status,
    document_number,
    internal_notes
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    c.filed_date + d.days_offset,
    d.title,
    d.description,
    d.public_entry,
    u_filing.id,
    u_clerk.id,
    'ACTIVE',
    'DISC-' || d.seq_num::text,
    d.internal_notes
FROM cases c
CROSS JOIN (VALUES
    (30, 1, 'Initial Disclosures',
     'Rule 26(a)(1) Initial Disclosures',
     'Parties exchanged initial disclosures pursuant to FRCP 26(a)(1)',
     'Initial disclosures completed for all parties'),
    (45, 2, 'First Request for Production',
     'Plaintiff''s First Request for Production of Documents',
     'USA served first request for production of documents on Google LLC',
     'High-priority document requests regarding search algorithms'),
    (60, 3, 'First Set of Interrogatories',
     'Plaintiff''s First Set of Interrogatories',
     'USA served first set of interrogatories on Google LLC',
     'Key questions about market dominance and advertising practices'),
    (75, 4, 'Discovery Response',
     'Defendant''s Response to First RFP',
     'Google LLC served responses to first request for production',
     'Large volume of documents expected - need review team'),
    (90, 5, 'Discovery Dispute Conference',
     'Discovery Dispute Conference Held',
     'Court held conference regarding scope of document production',
     'Judge ordered focused production schedule')
) as d(days_offset, seq_num, title, description, public_entry, internal_notes)
JOIN event_types et ON et.name = 'DISCOVERY'
JOIN users u_filing ON u_filing.username = 'pclement'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 29: Discovery Documents
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title,
    d.description,
    '/documents/' || c.case_number || '/discovery/' || lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || c.case_number || d.days_offset::text)::text,
    d.file_size,
    'PUBLIC',
    u.id,
    c.filed_date + d.days_offset,
    true,
    jsonb_build_object(
        'discovery_type', d.disc_type,
        'request_count', d.req_count,
        'response_deadline', (c.filed_date + d.days_offset + interval '30 days')::text,
        'confidentiality', d.confidentiality
    )
FROM cases c
CROSS JOIN (VALUES
    ('Initial Disclosures', 'Rule 26(a)(1) Initial Disclosures',
     30, 5000, 'INITIAL', 25, 'PUBLIC'),
    ('First RFP', 'First Request for Production of Documents',
     45, 8000, 'RFP', 150, 'CONFIDENTIAL'),
    ('First ROGs', 'First Set of Interrogatories',
     60, 3000, 'ROG', 25, 'PUBLIC'),
    ('RFP Responses', 'Responses to First RFP',
     75, 12000, 'RFP_RESPONSE', 150, 'CONFIDENTIAL'),
    ('Discovery Index', 'Index of Produced Documents',
     90, 1000, 'INDEX', NULL, 'PUBLIC')
) as d(title, description, days_offset, file_size, disc_type, req_count, confidentiality)
JOIN document_types dt ON dt.category = 'DISCOVERY'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 30: Scheduled Conferences
INSERT INTO calendar_events (
    title,
    description,
    event_type,
    priority,
    start_time,
    end_time,
    location_type,
    location_details,
    case_id,
    case_filed_date,
    organizer_id,
    participants,
    status,
    created_by
)
SELECT
    conf.title,
    conf.description,
    'HEARING',
    conf.priority,
    c.filed_date + conf.days_offset + '09:00:00'::interval,
    c.filed_date + conf.days_offset + '10:00:00'::interval,
    'COURTROOM',
    jsonb_build_object(
        'room', 'Courtroom 1',
        'floor', '3rd Floor',
        'building', 'Main Courthouse',
        'virtual_link', CASE WHEN conf.is_virtual THEN 'https://courts.gov/virtual/' || c.case_number ELSE NULL END
    ),
    c.id,
    c.filed_date,
    u.id,
    jsonb_build_array(
        jsonb_build_object('role', 'JUDGE', 'user_id', judge.id),
        jsonb_build_object('role', 'ATTORNEY', 'user_id', atty.id),
        jsonb_build_object('role', 'CLERK', 'user_id', clerk.id)
    ),
    'SCHEDULED',
    u.id
FROM cases c
CROSS JOIN (VALUES
    ('Initial Conference', 'Initial Scheduling Conference', 30, 'HIGH', false),
    ('Discovery Conference', 'Discovery Status and Planning', 60, 'NORMAL', true),
    ('Settlement Conference', 'Settlement Discussion', 90, 'HIGH', false),
    ('Final Pretrial Conference', 'Final Pretrial Conference', 120, 'HIGH', false),
    ('Motion Hearing', 'Hearing on Motion to Dismiss', 150, 'HIGH', false)
) as conf(title, description, days_offset, priority, is_virtual)
JOIN users u ON u.username = 'ckagan'
JOIN users judge ON judge.username = 'jroberts'
JOIN users atty ON atty.username = 'pclement'
JOIN users clerk ON clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 31: Detailed Motions and Responses
INSERT INTO motions (
    case_id,
    case_filed_date,
    motion_type_id,
    title,
    document_id,
    filed_by,
    filed_date,
    status,
    response_deadline,
    expedited,
    relief_requested,
    grounds
)
SELECT
    c.id,
    c.filed_date,
    mt.id,
    m.title,
    d.id,
    u.id,
    c.filed_date + m.days_offset,
    m.status,
    c.filed_date + m.days_offset + interval '21 days',
    m.is_expedited,
    m.relief,
    m.grounds
FROM cases c
CROSS JOIN (VALUES
    ('Motion to Dismiss', 45, 'PENDING', false,
     'Dismissal of Sherman Act claims',
     'Failure to state a claim upon which relief can be granted'),
    ('Motion to Compel Production', 75, 'PENDING', true,
     'Order compelling document production',
     'Inadequate response to discovery requests'),
    ('Motion for Protective Order', 90, 'GRANTED', false,
     'Entry of confidentiality order',
     'Protection of confidential business information'),
    ('Motion for Extension', 120, 'GRANTED', false,
     'Extension of discovery deadline',
     'Volume of documents requires additional time')
) as m(title, days_offset, status, is_expedited, relief, grounds)
JOIN motion_types mt ON mt.name = split_part(m.title, ' ', 2) || ' ' || split_part(m.title, ' ', 3)
JOIN users u ON u.username = 'pclement'
LEFT JOIN documents d ON d.case_id = c.id AND d.title LIKE '%' || split_part(m.title, ' ', 2) || '%'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 32: Expert Witnesses (through case_parties)
INSERT INTO case_parties (
    case_id,
    case_filed_date,
    party_type_id,
    name,
    is_lead,
    address,
    is_pro_se,
    individual_details
)
SELECT
    c.id,
    c.filed_date,
    pt.id,
    e.name,
    false,
    jsonb_build_object(
        'institution', e.institution,
        'city', e.city,
        'state', e.state
    ),
    false,
    jsonb_build_object(
        'expertise', e.expertise,
        'credentials', e.credentials,
        'prior_testimony', e.prior_cases
    )
FROM cases c
CROSS JOIN (VALUES
    ('Dr. John Economics', 'Harvard University', 'Cambridge', 'MA',
     'Antitrust Economics', 'Ph.D. Economics, Harvard', '15 federal cases'),
    ('Prof. Sarah Tech', 'Stanford University', 'Stanford', 'CA',
     'Computer Science', 'Ph.D. Computer Science, MIT', '8 federal cases'),
    ('Dr. Mark Market', 'University of Chicago', 'Chicago', 'IL',
     'Market Analysis', 'Ph.D. Business, Chicago', '20 federal cases')
) as e(name, institution, city, state, expertise, credentials, prior_cases)
JOIN party_types pt ON pt.name = 'EXPERT_WITNESS'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 33: Expert Reports and Documents
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title,
    d.description,
    '/documents/' || c.case_number || '/experts/' || lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || c.case_number || d.days_offset::text)::text,
    d.file_size,
    'CONFIDENTIAL',
    u.id,
    c.filed_date + d.days_offset,
    true,
    jsonb_build_object(
        'expert_name', d.expert_name,
        'report_type', d.report_type,
        'subject_matter', d.subject
    )
FROM cases c
CROSS JOIN (VALUES
    ('Economic Impact Analysis', 'Expert Report on Economic Impact',
     120, 15000, 'Dr. John Economics', 'INITIAL', 'Market Competition Analysis'),
    ('Technical Architecture Report', 'Expert Report on Search Technology',
     125, 12000, 'Prof. Sarah Tech', 'INITIAL', 'Search Algorithm Analysis'),
    ('Market Share Analysis', 'Expert Report on Market Definition',
     130, 10000, 'Dr. Mark Market', 'INITIAL', 'Relevant Market Analysis'),
    ('Rebuttal Economic Report', 'Rebuttal to Defense Economic Expert',
     150, 8000, 'Dr. John Economics', 'REBUTTAL', 'Economic Impact Rebuttal')
) as d(title, description, days_offset, file_size, expert_name, report_type, subject)
JOIN document_types dt ON dt.category = 'EXPERT_REPORT'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;


-- Part 34: Settlement Negotiations (through case_events with confidential status)
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status,
    internal_notes
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    c.filed_date + s.days_offset,
    s.title,
    s.description,
    s.public_entry,
    u_judge.id,
    u_clerk.id,
    'CONFIDENTIAL',
    s.internal_notes
FROM cases c
CROSS JOIN (VALUES
    (100, 'Settlement Conference Scheduled',
     'First Settlement Conference',
     'Settlement conference scheduled',
     'Initial settlement discussions to be held with magistrate judge'),
    (110, 'Settlement Conference Held',
     'Settlement Conference Results',
     'Settlement conference held; case ongoing',
     'Parties remain far apart on key terms; follow-up conference recommended'),
    (140, 'Mediation Referral',
     'Case Referred to Mediation',
     'Case referred to court-annexed mediation program',
     'Parties agreed to private mediation with Judge Smith (ret.)'),
    (170, 'Mediation Session',
     'First Mediation Session',
     'Mediation session held',
     'Progress made on technical compliance issues; monetary terms still disputed'),
    (200, 'Settlement Discussions',
     'Continued Settlement Negotiations',
     'Parties engaged in ongoing settlement discussions',
     'Term sheet in development; DOJ seeking additional injunctive provisions')
) as s(days_offset, title, description, public_entry, internal_notes)
JOIN event_types et ON et.name = 'STATUS_CHANGE'
JOIN users u_judge ON u_judge.username = 'jroberts'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 35: Case Milestones and Key Deadlines
INSERT INTO case_deadlines (
    case_id,
    case_filed_date,
    title,
    deadline_type,
    due_date,
    priority,
    status,
    reminder_days,
    created_by,
    description
)
SELECT
    c.id,
    c.filed_date,
    m.title,
    m.deadline_type,
    c.filed_date + m.days_offset,
    m.priority,
    CASE
        WHEN c.filed_date + m.days_offset < CURRENT_DATE THEN 'COMPLETED'
        ELSE 'PENDING'
    END,
    m.reminder_days,
    u.id,
    m.description
FROM cases c
CROSS JOIN (VALUES
    ('Initial Disclosures Due', 'DISCOVERY', 30, 'HIGH',
     ARRAY[7,3,1], 'Complete Rule 26(a)(1) initial disclosures'),
    ('Fact Discovery Cutoff', 'DISCOVERY', 180, 'HIGH',
     ARRAY[30,14,7], 'Complete all fact discovery'),
    ('Expert Reports Due', 'EXPERT', 210, 'HIGH',
     ARRAY[14,7,3], 'Plaintiff expert reports due'),
    ('Rebuttal Expert Reports', 'EXPERT', 240, 'HIGH',
     ARRAY[14,7,3], 'Defense expert reports due'),
    ('Discovery Completion', 'DISCOVERY', 270, 'HIGH',
     ARRAY[30,14,7], 'All discovery to be completed'),
    ('Summary Judgment Deadline', 'MOTION', 300, 'HIGH',
     ARRAY[30,14,7], 'Dispositive motions due'),
    ('Pretrial Conference', 'HEARING', 330, 'HIGH',
     ARRAY[14,7,3], 'Final pretrial conference'),
    ('Trial Start Date', 'TRIAL', 360, 'HIGH',
     ARRAY[30,14,7], 'Estimated trial start date')
) as m(title, deadline_type, days_offset, priority, reminder_days, description)
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 36: Milestone Document Requirements
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title,
    d.description,
    '/documents/' || c.case_number || '/milestones/' || lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || c.case_number || d.days_offset::text)::text,
    d.file_size,
    d.security_level,
    u.id,
    c.filed_date + d.days_offset,
    true,
    jsonb_build_object(
        'milestone_type', d.milestone_type,
        'requires_response', d.requires_response,
        'approval_required', d.approval_required
    )
FROM cases c
CROSS JOIN (VALUES
    ('Case Management Plan', 'Proposed case management plan and scheduling order',
     15, 1000, 'PUBLIC', 'SCHEDULING', true, true),
    ('Status Report', 'Joint status report on case progress',
     90, 2000, 'PUBLIC', 'STATUS', false, false),
    ('ADR Certification', 'Certification of ADR session completion',
     180, 500, 'PUBLIC', 'ADR', true, true),
    ('Pretrial Order', 'Proposed final pretrial order',
     330, 5000, 'PUBLIC', 'PRETRIAL', true, true)
) as d(title, description, days_offset, file_size, security_level, milestone_type, requires_response, approval_required)
JOIN document_types dt ON dt.category = 'ORDER'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;


-- Part 37: Security and Access Logs
INSERT INTO document_access_logs (
    document_id,
    user_id,
    access_type,
    access_timestamp,
    ip_address,
    user_agent,
    access_location,
    success,
    failure_reason
)
SELECT
    d.id,
    u.id,
    al.access_type,
    c.filed_date + al.days_offset + al.time_offset,
    al.ip_address::inet,
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    al.location,
    al.success,
    CASE WHEN al.success THEN NULL ELSE al.failure_reason END
FROM cases c
JOIN documents d ON d.case_id = c.id
CROSS JOIN (VALUES
    ('VIEW', 5, '09:00:00'::interval, '10.0.0.1', 'Office', true, NULL),
    ('DOWNLOAD', 5, '09:15:00'::interval, '10.0.0.1', 'Office', true, NULL),
    ('PRINT', 5, '09:30:00'::interval, '10.0.0.1', 'Office', false, 'Printer not authorized'),
    ('VIEW', 6, '10:00:00'::interval, '10.0.0.2', 'Remote', true, NULL),
    ('DOWNLOAD', 6, '10:15:00'::interval, '10.0.0.2', 'Remote', false, 'Permission denied')
) as al(access_type, days_offset, time_offset, ip_address, location, success, failure_reason)
JOIN users u ON u.username IN ('jroberts', 'ckagan', 'pclement')
WHERE c.case_number = '1:20-cv-03010'
AND d.title IN ('Complaint', 'Motion to Dismiss', 'Expert Report')
ON CONFLICT DO NOTHING;

-- Part 38: Court Orders and Opinions
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    o.title,
    o.description,
    '/documents/' || c.case_number || '/orders/' || lower(replace(o.title, ' ', '_')) || '.pdf',
    md5(o.title || c.case_number || o.days_offset::text)::text,
    o.file_size,
    'PUBLIC',
    u_judge.id,
    c.filed_date + o.days_offset,
    true,
    jsonb_build_object(
        'order_type', o.order_type,
        'requires_response', o.requires_response,
        'deadline_days', o.deadline_days,
        'precedential', o.precedential
    )
FROM cases c
CROSS JOIN (VALUES
    ('Initial Scheduling Order', 'Order setting case schedule and deadlines',
     15, 2000, 'SCHEDULING', false, NULL, false),
    ('Protective Order', 'Order governing confidential information',
     30, 3000, 'PROTECTIVE', false, NULL, false),
    ('Order on Motion to Dismiss', 'Opinion and Order on Motion to Dismiss',
     90, 15000, 'OPINION', true, 21, true),
    ('Discovery Order', 'Order resolving discovery disputes',
     120, 5000, 'DISCOVERY', true, 14, false),
    ('Summary Judgment Opinion', 'Opinion on Summary Judgment Motions',
     180, 25000, 'OPINION', true, 30, true)
) as o(title, description, days_offset, file_size, order_type, requires_response, deadline_days, precedential)
JOIN document_types dt ON dt.category = 'ORDER'
JOIN users u_judge ON u_judge.username = 'jroberts'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 39: Case Event Entries for Orders
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status,
    document_number
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    d.filed_date,
    'Order Filed: ' || d.title,
    d.description,
    'Court entered ' || d.title,
    d.filed_by,
    u_clerk.id,
    'ACTIVE',
    'ORD-' || ROW_NUMBER() OVER (PARTITION BY c.id ORDER BY d.filed_date)
FROM cases c
JOIN documents d ON d.case_id = c.id
JOIN document_types dt ON dt.id = d.document_type_id
JOIN event_types et ON et.name = 'ORDER'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
AND dt.category = 'ORDER'
ON CONFLICT DO NOTHING;

-- Part 40: Order Compliance Tracking (Fixed)
INSERT INTO case_deadlines (
    case_id,
    case_filed_date,
    title,
    deadline_type,
    due_date,  -- Ensure this is never null
    priority,
    status,
    reminder_days,
    created_by,
    description
)
SELECT
    c.id,
    c.filed_date,
    'Comply with ' || d.title,
    'ORDER_COMPLIANCE',
    COALESCE(
        d.filed_date + (COALESCE((d.metadata->>'deadline_days')::integer, 14) || ' days')::interval,
        d.filed_date + interval '14 days'  -- Default 14 days if no deadline specified
    ),
    'HIGH',
    'PENDING',
    ARRAY[1,3,7],
    u.id,
    'Deadline to comply with ' || d.title
FROM cases c
JOIN documents d ON d.case_id = c.id
JOIN document_types dt ON dt.id = d.document_type_id
JOIN users u ON u.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
AND dt.category = 'ORDER'
AND (d.metadata->>'requires_response')::boolean = true
ON CONFLICT DO NOTHING;



-- Part 41: Trial Exhibits
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    e.title,
    e.description,
    '/documents/' || c.case_number || '/exhibits/' || e.exhibit_number || '.pdf',
    md5(e.title || c.case_number || e.exhibit_number)::text,
    e.file_size,
    e.security_level,
    u.id,
    c.filed_date + interval '300 days',
    true,
    jsonb_build_object(
        'exhibit_number', e.exhibit_number,
        'exhibit_type', e.exhibit_type,
        'offered_by', e.offered_by,
        'witness', e.witness,
        'admission_status', 'PENDING'
    )
FROM cases c
CROSS JOIN (VALUES
    ('PX001', 'Market Share Analysis', 'Statistical analysis of search market share',
     5000, 'PUBLIC', 'DOCUMENT', 'PLAINTIFF', 'Dr. John Economics'),
    ('PX002', 'Technical Architecture Diagram', 'Google search system architecture',
     8000, 'CONFIDENTIAL', 'TECHNICAL', 'PLAINTIFF', 'Prof. Sarah Tech'),
    ('PX003', 'Internal Strategy Memo', 'Company strategic planning document',
     3000, 'HIGHLY_CONFIDENTIAL', 'BUSINESS', 'PLAINTIFF', 'Dr. Mark Market'),
    ('DX001', 'Competition Analysis', 'Analysis of search engine competition',
     4000, 'PUBLIC', 'DOCUMENT', 'DEFENDANT', 'Defense Expert 1'),
    ('DX002', 'User Choice Study', 'Study of user search engine preferences',
     6000, 'PUBLIC', 'RESEARCH', 'DEFENDANT', 'Defense Expert 2')
) as e(exhibit_number, title, description, file_size, security_level, exhibit_type, offered_by, witness)
JOIN document_types dt ON dt.category = 'EXHIBIT'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 42: Trial Witness Lists (Fixed)
INSERT INTO case_events (
    case_id,
    case_filed_date,
    event_type_id,
    event_date,
    title,
    description,
    public_entry,
    filed_by,
    entered_by,
    event_status,
    document_number
)
SELECT
    c.id,
    c.filed_date,
    et.id,
    c.filed_date + interval '290 days',
    w.title,
    w.description || ' - ' || w.witness_type || ' (' || w.est_time || ') - ' || w.subject,
    'Witness list filed: ' || w.party,
    u_filing.id,
    u_clerk.id,
    'ACTIVE',
    'WIT-' || ROW_NUMBER() OVER (ORDER BY w.order_num)
FROM cases c
CROSS JOIN (VALUES
    (1, 'Expert Witness List', 'List of expert witnesses',
     'EXPERT', '4 hours', 'Market Analysis', 'PLAINTIFF'),
    (2, 'Fact Witness List', 'List of fact witnesses',
     'FACT', '8 hours', 'Business Operations', 'PLAINTIFF'),
    (3, 'Corporate Representative List', 'List of corporate representatives',
     'CORPORATE', '6 hours', 'Company Practices', 'DEFENDANT')
) as w(order_num, title, description, witness_type, est_time, subject, party)
JOIN event_types et ON et.name = 'FILING'
JOIN users u_filing ON u_filing.username = 'pclement'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 43: Trial Preparation Documents
INSERT INTO documents (
    case_id,
    case_filed_date,
    document_type_id,
    title,
    description,
    storage_path,
    checksum,
    file_size,
    security_level,
    filed_by,
    filed_date,
    is_electronic,
    metadata
)
SELECT
    c.id,
    c.filed_date,
    dt.id,
    d.title,
    d.description,
    '/documents/' || c.case_number || '/trial_prep/' || lower(replace(d.title, ' ', '_')) || '.pdf',
    md5(d.title || c.case_number)::text,
    d.file_size,
    'CONFIDENTIAL',
    u.id,
    c.filed_date + interval '280 days',
    true,
    jsonb_build_object(
        'document_type', d.doc_type,
        'trial_phase', d.trial_phase,
        'internal_use', true
    )
FROM cases c
CROSS JOIN (VALUES
    ('Preliminary Jury Instructions', 'Proposed preliminary jury instructions',
     5000, 'INSTRUCTIONS', 'PRELIMINARY'),
    ('Opening Statement Outline', 'Detailed opening statement outline',
     3000, 'ARGUMENT', 'OPENING'),
    ('Direct Examination Outlines', 'Witness examination outlines',
     8000, 'EXAMINATION', 'TESTIMONY'),
    ('Closing Argument Notes', 'Detailed closing argument outline',
     4000, 'ARGUMENT', 'CLOSING'),
    ('Trial Brief', 'Comprehensive trial brief',
     12000, 'BRIEF', 'ALL')
) as d(title, description, file_size, doc_type, trial_phase)
JOIN document_types dt ON dt.category = 'TRIAL'
JOIN users u ON u.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;

-- Part 44: Trial Schedule
INSERT INTO calendar_events (
    title,
    description,
    event_type,
    priority,
    start_time,
    end_time,
    location_type,
    location_details,
    case_id,
    case_filed_date,
    organizer_id,
    participants,
    status,
    created_by   -- Adding the missing created_by field
)
SELECT
    t.title,
    t.description,
    'TRIAL',
    'HIGH',
    c.filed_date + interval '365 days' + t.day_offset,
    c.filed_date + interval '365 days' + t.day_offset + interval '6 hours',
    'COURTROOM',
    jsonb_build_object(
        'courtroom', 'Main Courtroom',
        'floor', '3rd Floor',
        'building', 'Federal Courthouse',
        'special_equipment', t.equipment
    ),
    c.id,
    c.filed_date,
    u_judge.id,
    jsonb_build_array(
        jsonb_build_object('role', 'JUDGE', 'user_id', u_judge.id),
        jsonb_build_object('role', 'ATTORNEY', 'user_id', u_atty.id)
    ),
    'SCHEDULED',
    u_judge.id    -- Setting created_by to the judge's user ID
FROM cases c
CROSS JOIN (VALUES
    ('Trial Day 1 - Openings', 'Opening statements and initial witnesses',
     interval '0 days', ARRAY['Audio System', 'Evidence Display']),
    ('Trial Day 2 - Plaintiff Case', 'Plaintiff''s key expert testimony',
     interval '1 days', ARRAY['Audio System', 'Video Conference']),
    ('Trial Day 3 - Plaintiff Case', 'Plaintiff''s fact witnesses',
     interval '2 days', ARRAY['Audio System']),
    ('Trial Day 4 - Defense Case', 'Defense expert testimony',
     interval '3 days', ARRAY['Audio System', 'Evidence Display']),
    ('Trial Day 5 - Closings', 'Closing arguments and jury instructions',
     interval '4 days', ARRAY['Audio System'])
) as t(title, description, day_offset, equipment)
JOIN users u_judge ON u_judge.username = 'jroberts'
JOIN users u_atty ON u_atty.username = 'pclement'
WHERE c.case_number = '1:20-cv-03010'
ON CONFLICT DO NOTHING;


-- Fee Schedule Data
INSERT INTO fee_schedules (filing_type, fee_amount, effective_date, waiver_eligible) VALUES
('NEW_CASE', 400.00, '2024-01-01', true),
('APPEAL', 505.00, '2024-01-01', true),
('PRO_HAC_VICE', 200.00, '2024-01-01', false),
('MOTION', 50.00, '2024-01-01', true);

-- Transaction Records
INSERT INTO financial_transactions (
    case_id,
    case_filed_date,
    transaction_type,
    amount,
    payment_method,
    status,
    payer_id,
    received_by
) SELECT
    c.id,
    c.filed_date,
    'FILING_FEE',
    400.00,
    'CREDIT_CARD',
    'COMPLETED',
    u_payer.id,
    u_clerk.id
FROM cases c
JOIN users u_payer ON u_payer.username = 'pclement'
JOIN users u_clerk ON u_clerk.username = 'ckagan'
WHERE c.case_number = '1:20-cv-03010';


-- Service Recipients
INSERT INTO service_recipients (
    case_id,
    case_filed_date,
    recipient_type,
    party_id,
    email,
    service_preference
) SELECT
    c.id,
    c.filed_date,
    'PARTY',
    cp.id,
    'service@google.com',
    'ELECTRONIC'
FROM cases c
JOIN case_parties cp ON c.id = cp.case_id
WHERE c.case_number = '1:20-cv-03010';

-- Service Transactions
INSERT INTO service_transactions (
    document_id,
    recipient_id,
    service_method,
    status,
    delivery_confirmation
) SELECT
    d.id,
    sr.id,
    'EMAIL',
    'COMPLETED',
    md5(random()::text)
FROM documents d
JOIN service_recipients sr ON d.case_id = sr.case_id;


-- API Keys
INSERT INTO api_keys (
    user_id,
    key_hash,
    name,
    permissions,
    expires_at
) SELECT
    u.id,
    md5(random()::text),
    'Production API Key',
    '{"read": true, "write": true}'::jsonb,
    CURRENT_TIMESTAMP + interval '1 year'
FROM users u
WHERE u.username IN ('pclement', 'ckagan');

-- API Request Logs
INSERT INTO api_requests (
    api_key_id,
    endpoint,
    method,
    request_body,
    response_status,
    processing_time
) SELECT
    ak.id,
    '/api/v1/cases',
    'GET',
    '{"case_number": "1:20-cv-03010"}'::jsonb,
    200,
    random() * 1000
FROM api_keys ak;


-- API Keys for existing users
INSERT INTO api_keys (
    user_id,
    key_hash,
    name,
    permissions,
    status,
    expires_at,
    last_used_at,
    created_by,
    created_at
)
SELECT
    u.id,
    md5(u.username || '-api-key'),  -- Simulate key hash
    CASE
        WHEN u.username = 'pclement' THEN 'Production API Key'
        WHEN u.username = 'tgoldstein' THEN 'Development API Key'
        ELSE 'Test API Key'
    END,
    CASE
        WHEN r.name = 'attorney' THEN '{"cases": {"read": true, "write": true}, "documents": {"read": true, "write": true}}'::jsonb
        WHEN r.name = 'judge' THEN '{"cases": {"read": true}, "documents": {"read": true}, "orders": {"write": true}}'::jsonb
        WHEN r.name = 'clerk' THEN '{"cases": {"read": true, "write": true}, "calendar": {"read": true, "write": true}}'::jsonb
    END,
    'ACTIVE',
    CURRENT_TIMESTAMP + interval '1 year',
    NOW() - (random() * interval '24 hours'),
    (SELECT id FROM users WHERE username = 'msmith'),  -- Admin creates all keys
    NOW() - (random() * interval '90 days')
FROM users u
JOIN roles r ON u.role_id = r.id
WHERE u.username IN ('pclement', 'tgoldstein', 'nkatyal', 'ckagan')
ON CONFLICT DO NOTHING;

-- Rate limits for each API key
INSERT INTO api_rate_limits (
    api_key_id,
    window_size,
    max_requests,
    current_count,
    window_start
)
SELECT
    id,
    CASE
        WHEN name LIKE '%Production%' THEN interval '1 minute'
        WHEN name LIKE '%Development%' THEN interval '1 hour'
        ELSE interval '1 day'
    END,
    CASE
        WHEN name LIKE '%Production%' THEN 1000
        WHEN name LIKE '%Development%' THEN 500
        ELSE 100
    END,
    floor(random() * 50),  -- Random current count
    NOW() - interval '30 minutes'
FROM api_keys
ON CONFLICT DO NOTHING;

-- API Requests mock data
WITH timeframes AS (
    SELECT generate_series(
        NOW() - interval '7 days',
        NOW(),
        interval '5 minutes'
    ) as request_time
),
endpoints AS (
    SELECT unnest(ARRAY[
        '/api/v1/cases',
        '/api/v1/cases/{id}',
        '/api/v1/documents',
        '/api/v1/calendar',
        '/api/v1/orders'
    ]) as endpoint,
    unnest(ARRAY[
        'GET',
        'GET',
        'POST',
        'GET',
        'POST'
    ]) as method
),
sample_requests AS (
    SELECT
        ak.id as api_key_id,
        t.request_time as request_timestamp,
        e.endpoint,
        e.method,
        CASE WHEN random() < 0.9 THEN 200 ELSE (ARRAY[400, 401, 403, 404, 500])[floor(random() * 5 + 1)] END as response_status,
        floor(random() * 500 + 10) as processing_time
    FROM timeframes t
    CROSS JOIN endpoints e
    CROSS JOIN api_keys ak
    WHERE random() < 0.1  -- Only generate requests for 10% of timeframes to avoid too much data
)
INSERT INTO api_requests (
    api_key_id,
    endpoint,
    method,
    request_body,
    response_status,
    response_body,
    ip_address,
    user_agent,
    processing_time,
    error_message,
    request_timestamp
)
SELECT
    api_key_id,
    endpoint,
    method,
    CASE
        WHEN method = 'POST' THEN jsonb_build_object('data', 'sample request data')
        ELSE NULL
    END,
    response_status,
    CASE
        WHEN response_status = 200 THEN jsonb_build_object('status', 'success')
        ELSE jsonb_build_object('error', 'Sample error message')
    END,
    (ARRAY['10.0.0.1', '10.0.0.2', '192.168.1.100'])[floor(random() * 3 + 1)]::inet,
    'Mozilla/5.0 (compatible; Court API Client/1.0)',
    processing_time,
    CASE
        WHEN response_status != 200 THEN 'Sample error for status ' || response_status
        ELSE NULL
    END,
    request_timestamp
FROM sample_requests
ON CONFLICT DO NOTHING;

-- Create a revoked key to demonstrate key lifecycle
INSERT INTO api_keys (
    user_id,
    key_hash,
    name,
    permissions,
    status,
    expires_at,
    last_used_at,
    created_by,
    revoked_at,
    revoked_by,
    revocation_reason,
    created_at
)
SELECT
    u1.id,
    md5('revoked-key'),
    'Revoked Test Key',
    '{"cases": {"read": true}}'::jsonb,
    'REVOKED',
    CURRENT_TIMESTAMP + interval '1 year',
    NOW() - interval '5 days',
    u2.id,
    NOW() - interval '1 day',
    u2.id,
    'Security policy update required key rotation',
    NOW() - interval '30 days'
FROM users u1
CROSS JOIN users u2
WHERE u1.username = 'tgoldstein'
AND u2.username = 'msmith'
ON CONFLICT DO NOTHING;

-- Update last_used_at for active keys based on their latest request
UPDATE api_keys ak
SET last_used_at = (
    SELECT MAX(request_timestamp)
    FROM api_requests ar
    WHERE ar.api_key_id = ak.id
)
WHERE EXISTS (
    SELECT 1
    FROM api_requests ar
    WHERE ar.api_key_id = ak.id
);
