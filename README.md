# Lexodus

# Deps
## Install spin
```bash
brew install brew/spin
```
## Install Leptos
```bash
cargo install --locked leptos
```
## Add WASM target
```bash
rustup target add wasm32-unknown-unknown &&\
rustup target add wasm32-wasi
```

## Run the project
```bash
spin up --build
```

# Architecture

```
src/
├── server/
│   ├── commands/
│   │   ├── case_management.rs
│   │   ├── document_management.rs
│   │   ├── user_management.rs
│   │   ├── chat.rs
│   │   ├── activity_tracking.rs
│   │   └── financial.rs
│   │
│   ├── queries/
│   │   ├── case_management.rs
│   │   ├── document_management.rs
│   │   ├── user_management.rs
│   │   ├── chat.rs
│   │   ├── activity_tracking.rs
│   │   └── financial.rs
│   │
│   ├── models/
│   │   ├── case.rs
│   │   ├── document.rs
│   │   ├── user.rs
│   │   ├── chat.rs
│   │   └── activity.rs
│   │
│   └── db/
│       └── connection.rs
│
├── components/
│   ├── case_management/
│   ├── document_management/
│   ├── user_management/
│   ├── chat/
│   └── activity_tracking/
│
└── pages/
    ├── case_management.rs
    ├── document_management.rs
    ├── user_management.rs
    ├── chat.rs
    └── activity_tracking.rs
```

Now, let's break down the content of each file in the `server/commands/` and `server/queries/` directories:

1. `server/commands/case_management.rs`:
```rust
pub async fn create_case(case: Case) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_case_details(case: Case) -> Result<String, ServerFnError> { /* ... */ }
pub async fn delete_case(case_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_case_status(case_id: i64, new_status: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn add_case_note(case_id: i64, note: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn add_party_to_case(case_id: i64, party: Party) -> Result<String, ServerFnError> { /* ... */ }
pub async fn remove_party_from_case(case_id: i64, party_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn schedule_hearing(case_id: i64, hearing: Hearing) -> Result<String, ServerFnError> { /* ... */ }
pub async fn cancel_hearing(hearing_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn file_motion(case_id: i64, motion: Motion) -> Result<String, ServerFnError> { /* ... */ }
pub async fn file_pleading(case_id: i64, pleading: Pleading) -> Result<String, ServerFnError> { /* ... */ }
pub async fn create_discovery_request(case_id: i64, request: DiscoveryRequest) -> Result<String, ServerFnError> { /* ... */ }
pub async fn record_judgment(case_id: i64, judgment: Judgment) -> Result<String, ServerFnError> { /* ... */ }
pub async fn file_appeal(case_id: i64, appeal: Appeal) -> Result<String, ServerFnError> { /* ... */ }
pub async fn transfer_case(case_id: i64, to_court_id: i64, reason: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn assign_case(case_id: i64, judge_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn add_case_tag(case_id: i64, tag: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn remove_case_tag(case_id: i64, tag: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn link_related_case(case_id: i64, related_case_id: i64, relationship_type: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn add_case_flag(case_id: i64, flag_type: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn remove_case_flag(case_id: i64, flag_type: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn archive_case(case_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn retrieve_archived_case(case_id: i64) -> Result<String, ServerFnError> { /* ... */ }
```

2. `server/queries/case_management.rs`:
```rust
pub async fn get_case_details(case_id: i64) -> Result<Case, ServerFnError> { /* ... */ }
pub async fn get_case_status_history(case_id: i64) -> Result<Vec<(String, String)>, ServerFnError> { /* ... */ }
pub async fn get_case_parties(case_id: i64) -> Result<Vec<Party>, ServerFnError> { /* ... */ }
pub async fn get_case_hearings(case_id: i64) -> Result<Vec<Hearing>, ServerFnError> { /* ... */ }
pub async fn get_case_motions(case_id: i64) -> Result<Vec<Motion>, ServerFnError> { /* ... */ }
pub async fn get_case_pleadings(case_id: i64) -> Result<Vec<Pleading>, ServerFnError> { /* ... */ }
pub async fn get_case_discovery(case_id: i64) -> Result<Vec<DiscoveryRequest>, ServerFnError> { /* ... */ }
pub async fn get_case_judgments(case_id: i64) -> Result<Vec<Judgment>, ServerFnError> { /* ... */ }
pub async fn get_case_appeals(case_id: i64) -> Result<Vec<Appeal>, ServerFnError> { /* ... */ }
pub async fn get_case_transfer_history(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> { /* ... */ }
pub async fn get_case_tags(case_id: i64) -> Result<Vec<String>, ServerFnError> { /* ... */ }
pub async fn get_related_cases(case_id: i64) -> Result<Vec<(i64, String, String)>, ServerFnError> { /* ... */ }
pub async fn get_case_flags(case_id: i64) -> Result<Vec<String>, ServerFnError> { /* ... */ }
pub async fn search_cases(query: String, filters: Vec<(String, String)>) -> Result<Vec<Case>, ServerFnError> { /* ... */ }
pub async fn generate_case_status_report(start_date: String, end_date: String) -> Result<Vec<(String, i64)>, ServerFnError> { /* ... */ }
pub async fn get_case_activity_logs(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> { /* ... */ }
pub async fn get_case_timeline(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> { /* ... */ }
```

3. `server/commands/document_management.rs`:
```rust
pub async fn upload_document(case_id: i64, document: Document) -> Result<String, ServerFnError> { /* ... */ }
pub async fn seal_document(document_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn unseal_document(document_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn sign_document(user_id: i64, document_id: i64, signature: String) -> Result<String, ServerFnError> { /* ... */ }
```

4. `server/queries/document_management.rs`:
```rust
pub async fn get_case_documents(case_id: i64) -> Result<Vec<Document>, ServerFnError> { /* ... */ }
pub async fn verify_signature(document_id: i64, signature: String) -> Result<bool, ServerFnError> { /* ... */ }
```

5. `server/commands/user_management.rs`:
```rust
pub async fn set_notification_preference(user_id: i64, notification_type: String, enabled: bool) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_user_presence(user_id: i64, status: String) -> Result<String, ServerFnError> { /* ... */ }
```

6. `server/queries/user_management.rs`:
```rust
pub async fn get_user_permissions(user_id: i64) -> Result<Vec<String>, ServerFnError> { /* ... */ }
pub async fn get_user_presence(user_id: i64) -> Result<(String, String), ServerFnError> { /* ... */ }
```

7. `server/commands/chat.rs`:
```rust
pub async fn create_chat_room(case_id: i64, name: String) -> Result<i64, ServerFnError> { /* ... */ }
pub async fn send_chat_message(chat_room_id: i64, user_id: i64, content: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn join_chat_room(chat_room_id: i64, user_id: i64) -> Result<String, ServerFnError> { /* ... */ }
pub async fn leave_chat_room(chat_room_id: i64, user_id: i64) -> Result<String, ServerFnError> { /* ... */ }
```

8. `server/queries/chat.rs`:
```rust
pub async fn get_chat_messages(chat_room_id: i64, limit: i64) -> Result<Vec<(i64, String, String, String)>, ServerFnError> { /* ... */ }
```

9. `server/commands/activity_tracking.rs`:
```rust
pub async fn log_user_activity(user_id: i64, action_type: String, details: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn track_user_action(user_id: i64, action_type: String) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_user_last_action(user_id: i64, action_type: String) -> Result<String, ServerFnError> { /* ... */ }
```

10. `server/queries/activity_tracking.rs`:
```rust
pub async fn get_user_activities(user_id: i64, limit: i64) -> Result<Vec<(String, String, String)>, ServerFnError> { /* ... */ }
pub async fn get_user_frequent_actions(user_id: i64) -> Result<Vec<(String, i64)>, ServerFnError> { /* ... */ }
pub async fn get_user_last_actions(user_id: i64) -> Result<Vec<(String, String)>, ServerFnError> { /* ... */ }
```

11. `server/commands/financial.rs`:
```rust
pub async fn record_payment(case_id: i64, transaction: FinancialTransaction) -> Result<String, ServerFnError> { /* ... */ }
```

12. `server/queries/financial.rs`:
```rust
pub async fn get_case_financials(case_id: i64) -> Result<Vec<FinancialTransaction>, ServerFnError> { /* ... */ }
```

This structure separates the commands (which modify state) from the queries (which read state) for each major area of functionality. It will make it easier to maintain and scale your application as it grows. You can import these functions into your Leptos components as needed to implement the user interface for each feature.
