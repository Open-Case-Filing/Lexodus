src/
├── app.rs
├── application
│   ├── commands
│   │   ├── create_case.rs
│   │   ├── mod.rs
│   │   └── update_case.rs
│   ├── mod.rs
│   ├── queries
│   │   ├── get_case.rs
│   │   ├── get_case_notifications.rs
│   │   ├── get_court_orders.rs
│   │   ├── get_filing_deadlines.rs
│   │   ├── get_hearing_schedules.rs
│   │   └── mod.rs
│   └── services
│       ├── ai_service.rs
│       ├── case_service.rs
│       └── mod.rs
├── domain
│   ├── mod.rs
│   ├── models
│   │   ├── case.rs
│   │   ├── case_notification.rs
│   │   ├── civil_case.rs
│   │   ├── court_order.rs
│   │   ├── filing_deadline.rs
│   │   ├── hearing_schedule.rs
│   │   ├── mod.rs
│   │   ├── recent_case_activity.rs
│   │   └── user.rs
│   └── repositories
│       ├── case_repository.rs
│       ├── mod.rs
│       └── user_repository.rs
├── infrastructure
│   ├── di
│   │   ├── README.md
│   │   ├── container.rs
│   │   └── mod.rs
│   ├── logging
│   │   ├── README.md
│   │   ├── mod.rs
│   │   └── security_logger.rs
│   ├── mod.rs
│   ├── repositories
│   │   ├── mod.rs
│   │   ├── pg_case_repository.rs
│   │   └── pg_user_acitivity.rs
│   └── server
│       ├── errors.rs
│       ├── functions
│       │   ├── case_functions.rs
│       │   ├── mod.rs
│       │   └── rate_limiting.rs
│       ├── middleware
│       │   ├── auth.rs
│       │   ├── cors.rs
│       │   └── mod.rs
│       └── mod.rs
├── lib.rs
├── main.rs
├── mod.rs
├── presentation
│   ├── components
│   │   ├── action_menu.rs
│   │   ├── case_notifications.rs
│   │   ├── court_orders.rs
│   │   ├── filing_deadlines.rs
│   │   ├── footer.rs
│   │   ├── hearing_schedules.rs
│   │   ├── mod.rs
│   │   ├── nav.rs
│   │   ├── recent_case_activity.rs
│   │   └── search_bar.rs
│   ├── layouts
│   │   ├── default.rs
│   │   ├── mod.rs
│   │   └── wide.rs
│   ├── mod.rs
│   └── pages
│       ├── activity.rs
│       ├── case_management
│       │   ├── create.rs
│       │   └── mod.rs
│       ├── changelog.rs
│       ├── homepage.rs
│       ├── login.rs
│       ├── mod.rs
│       └── user_management
│           ├── create.rs
│           └── mod.rs
└── server.rs

21 directories, 72 files
