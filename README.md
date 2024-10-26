# Lexodus

## Mission
Lexodus aims to revolutionize judicial systems worldwide by providing a modern, accessible, and efficient case management platform. In many jurisdictions, access to justice is hampered by outdated technology and paper-based processes. This project bridges that gap by offering a comprehensive digital solution that serves courts, legal professionals, and citizens alike.

## Impact

### Access to Justice
- Reduces barriers to legal services through digital accessibility
- Enables remote court proceedings and document management
- Streamlines case filing and tracking for self-represented litigants
- Makes legal processes more transparent and understandable

### Judicial Efficiency
- Modernizes court operations with paperless workflows
- Reduces case backlogs through better management tools
- Enables data-driven decision making for resource allocation
- Supports remote and hybrid court operations

### Global Standards
- Open-source platform adaptable to different legal systems
- Built with international best practices for court technology
- Supports multiple languages and jurisdictions
- Promotes transparency and accountability in judicial processes

## Prerequisites

Before starting, ensure you have the following installed:

1. **Rust and Cargo**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Required Rust Targets**
```bash
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

3. **Leptos**
```bash
cargo install --locked leptos-cli
```

4. **Spin**
- macOS:
  ```bash
  brew install fermyon/tap/spin
  ```
- Windows/Linux:
  Visit [Spin Installation Guide](https://developer.fermyon.com/spin/install)

5. **Cargo Make** (Task Runner)
```bash
cargo install --force cargo-make
```

## Database Setup

Start the PostgreSQL database using Docker:
```bash
docker run --name ocfs-pg \
  -e POSTGRES_USER=app_user \
  -e POSTGRES_PASSWORD=dev_only_pwd \
  -e POSTGRES_DB=app_db \
  -p 5432:5432 \
  -d postgres:latest
```

## Development

### Available Commands

All commands use `cargo make` for consistency across platforms:

Unix/Linux/Mac:
```bash
cargo make fmt     # Format code
cargo make clean   # Clean build artifacts
cargo make up      # Build project
cargo make test    # Run tests
cargo make watch   # Start dev server with hot-reload
cargo make deploy  # Deploy application
cargo make run     # Run all tasks (fmt, up, test)
```

Windows:
```powershell
cargo make fmt        # Format code
cargo make clean      # Clean build artifacts
cargo make up-win     # Build project
cargo make test       # Run tests
cargo make watch-win  # Start dev server with hot-reload
cargo make deploy-win # Deploy application
cargo make run-win    # Run all tasks (fmt, up, test)
```

### Quick Start

1. **Clone and Enter Project**
```bash
git clone <repository-url>
cd lexodus
```

2. **Start Database**
```bash
docker run --name ocfs-pg -e POSTGRES_USER=app_user -e POSTGRES_PASSWORD=dev_only_pwd -e POSTGRES_DB=app_db -p 5432:5432 -d postgres:latest
```

3. **Development Mode**
Unix/Linux/Mac:
```bash
cargo make watch
```
Windows:
```powershell
cargo make watch-win
```

## Core Features

### Case Management
- Comprehensive case lifecycle management
- Document filing and tracking
- Hearing scheduling and management
- Party management and notifications

### Document Management
- Secure document storage and retrieval
- Digital signing capabilities
- Access control and permissions
- Document sealing and unsealing

### User Management
- Role-based access control
- Profile and preference management
- Activity tracking
- Notification management

### Communication
- Real-time chat for case participants
- Secure messaging system
- Notification preferences
- Activity tracking and auditing

### Financial Management
- Payment processing
- Financial transaction tracking
- Fee management
- Payment history

Before starting, ensure you have the following installed:

1. **Rust and Cargo**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Required Rust Targets**
```bash
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

3. **Leptos CLI**
```bash
cargo install cargo-leptos --locked
```

4. **Spin**
- macOS:
  ```bash
  brew install fermyon/tap/spin
  ```
- Windows/Linux:
  Visit [Spin Installation Guide](https://developer.fermyon.com/spin/install)

5. **Cargo Make** (Task Runner)
```bash
cargo install --force cargo-make
```

[Rest of the README remains the same...]

## Database Setup

Start the PostgreSQL database using Docker:
```bash
docker run --name ocfs-pg \
  -e POSTGRES_USER=app_user \
  -e POSTGRES_PASSWORD=dev_only_pwd \
  -e POSTGRES_DB=app_db \
  -p 5432:5432 \
  -d postgres:latest
```

## Development

### Available Commands

All commands use `cargo make` for consistency across platforms:

Unix/Linux/Mac:
```bash
cargo make fmt     # Format code
cargo make clean   # Clean build artifacts
cargo make up      # Build project
cargo make test    # Run tests
cargo make watch   # Start dev server with hot-reload
cargo make deploy  # Deploy application
cargo make run     # Run all tasks (fmt, up, test)
```

Windows:
```powershell
cargo make fmt        # Format code
cargo make clean      # Clean build artifacts
cargo make up-win     # Build project
cargo make test       # Run tests
cargo make watch-win  # Start dev server with hot-reload
cargo make deploy-win # Deploy application
cargo make run-win    # Run all tasks (fmt, up, test)
```

### Quick Start

1. **Clone and Enter Project**
```bash
git clone git@github.com:Open-Case-Filing/Lexodus.git
cd lexodus
```

2. **Start Database**
```bash
docker run --name ocfs-pg -e POSTGRES_USER=app_user -e POSTGRES_PASSWORD=dev_only_pwd -e POSTGRES_DB=app_db -p 5432:5432 -d postgres:latest
```

3. **Development Mode**
Unix/Linux/Mac:
```bash
cargo make watch
```
Windows:
```powershell
cargo make watch-win
```

## Project Structure

```
src/
├── server/
│   ├── commands/           # State-modifying operations
│   ├── queries/            # Read-only operations
│   ├── models/            # Data structures
│   └── db/               # Database connections
│
├── components/           # Reusable UI components
│   ├── case_management/
│   ├── document_management/
│   ├── user_management/
│   ├── chat/
│   └── activity_tracking/
│
└── pages/               # Page components
    ├── case_management.rs
    ├── document_management.rs
    ├── user_management.rs
    ├── chat.rs
    └── activity_tracking.rs
```

## License

This project is licensed under the [MIT License](LICENSE).

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

pub async fn update_user_settings(user_id: i64, settings: UserSettings) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_user_profile(user_id: i64, profile: UserProfile) -> Result<String, ServerFnError> { /* ... */ }
pub async fn update_user_password(user_id: i64, old_password: String, new_password: String) -> Result<String, ServerFnError> { /* ... */ }

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
