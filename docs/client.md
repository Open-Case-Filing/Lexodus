1. Case Management UI

```rust
// src/components/case_management/case_list.rs
use leptos::*;
use crate::server::queries::case_management::search_cases;

#[component]
pub fn CaseList() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let cases = create_resource(move || search_query.get(), |query| search_cases(query, vec![]));

    view! {
        <div>
            <h2>"Case List"</h2>
            <input
                type="text"
                placeholder="Search cases..."
                on:input=move |ev| set_search_query(event_target_value(&ev))
            />
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || cases.get().map(|result| match result {
                    Ok(case_list) => view! {
                        <ul>
                            {case_list.into_iter().map(|case| view! {
                                <li>{case.case_number} - {case.title}</li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    },
                    Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
                })}
            </Suspense>
        </div>
    }
}
```

```rust
// src/components/case_management/case_details.rs
use leptos::*;
use crate::server::queries::case_management::get_case_details;
use crate::server::commands::case_management::update_case_status;

#[component]
pub fn CaseDetails(case_id: i64) -> impl IntoView {
    let case = create_resource(move || case_id, get_case_details);
    let (status, set_status) = create_signal(String::new());

    let update_status = create_action(move |(case_id, new_status): &(i64, String)| {
        update_case_status(*case_id, new_status.clone())
    });

    view! {
        <div>
            <h2>"Case Details"</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || case.get().map(|result| match result {
                    Ok(case) => view! {
                        <div>
                            <p>"Case Number: " {&case.case_number}</p>
                            <p>"Title: " {&case.title}</p>
                            <p>"Status: " {&case.status}</p>
                            <p>"Filed Date: " {&case.filed_date}</p>
                            <input
                                type="text"
                                placeholder="New status"
                                on:input=move |ev| set_status(event_target_value(&ev))
                            />
                            <button
                                on:click=move |_| update_status.dispatch((case_id, status.get()))
                            >
                                "Update Status"
                            </button>
                        </div>
                    },
                    Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
                })}
            </Suspense>
        </div>
    }
}
```

2. Document Management UI

```rust
// src/components/document_management/document_list.rs
use leptos::*;
use crate::server::queries::document_management::get_case_documents;

#[component]
pub fn DocumentList(case_id: i64) -> impl IntoView {
    let documents = create_resource(move || case_id, get_case_documents);

    view! {
        <div>
            <h2>"Case Documents"</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || documents.get().map(|result| match result {
                    Ok(doc_list) => view! {
                        <ul>
                            {doc_list.into_iter().map(|doc| view! {
                                <li>{doc.title} - {doc.file_path}</li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    },
                    Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
                })}
            </Suspense>
        </div>
    }
}
```

```rust
// src/components/document_management/upload_document.rs
use leptos::*;
use crate::server::commands::document_management::upload_document;
use crate::server::models::document::Document;

#[component]
pub fn UploadDocument(case_id: i64) -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (file_path, set_file_path) = create_signal(String::new());

    let upload = create_action(move |_| {
        let document = Document {
            id: 0, // This will be set by the database
            title: title.get(),
            file_path: file_path.get(),
        };
        upload_document(case_id, document)
    });

    view! {
        <div>
            <h2>"Upload Document"</h2>
            <input
                type="text"
                placeholder="Document Title"
                on:input=move |ev| set_title(event_target_value(&ev))
            />
            <input
                type="text"
                placeholder="File Path"
                on:input=move |ev| set_file_path(event_target_value(&ev))
            />
            <button on:click=move |_| upload.dispatch(())>
                "Upload"
            </button>
            <p>{move || upload.value().get().map(|result| match result {
                Ok(message) => message,
                Err(e) => format!("Error: {}", e),
            })}</p>
        </div>
    }
}
```

3. Chat UI

```rust
// src/components/chat/chat_room.rs
use leptos::*;
use crate::server::queries::chat::get_chat_messages;
use crate::server::commands::chat::send_chat_message;

#[component]
pub fn ChatRoom(chat_room_id: i64, user_id: i64) -> impl IntoView {
    let (message, set_message) = create_signal(String::new());
    let messages = create_resource(move || chat_room_id, |id| get_chat_messages(id, 50));

    let send = create_action(move |_| {
        send_chat_message(chat_room_id, user_id, message.get())
    });

    view! {
        <div>
            <h2>"Chat Room"</h2>
            <div style="height: 400px; overflow-y: scroll;">
                <Suspense fallback=move || view! { <p>"Loading messages..."</p> }>
                    {move || messages.get().map(|result| match result {
                        Ok(msg_list) => view! {
                            <ul>
                                {msg_list.into_iter().map(|(id, username, content, timestamp)| view! {
                                    <li>{username}: {content} - {timestamp}</li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        },
                        Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
                    })}
                </Suspense>
            </div>
            <input
                type="text"
                placeholder="Type your message..."
                on:input=move |ev| set_message(event_target_value(&ev))
            />
            <button on:click=move |_| send.dispatch(())>
                "Send"
            </button>
        </div>
    }
}
```

These components provide a basic UI for case management, document management, and chat functionality. You can further customize and style these components to fit your specific needs and design preferences.

To use these components in your main application, you might create a layout like this:

```rust
// src/pages/main_dashboard.rs
use leptos::*;
use crate::components::case_management::{CaseList, CaseDetails};
use crate::components::document_management::{DocumentList, UploadDocument};
use crate::components::chat::ChatRoom;

#[component]
pub fn MainDashboard() -> impl IntoView {
    let (selected_case_id, set_selected_case_id) = create_signal(None::<i64>);
    let (selected_chat_room_id, set_selected_chat_room_id) = create_signal(None::<i64>);
    let user_id = 1; // This should be dynamically set based on the logged-in user

    view! {
        <div class="dashboard">
            <div class="sidebar">
                <CaseList />
            </div>
            <div class="main-content">
                {move || selected_case_id.get().map(|id| view! {
                    <CaseDetails case_id=id />
                    <DocumentList case_id=id />
                    <UploadDocument case_id=id />
                })}
            </div>
            <div class="chat-sidebar">
                {move || selected_chat_room_id.get().map(|id| view! {
                    <ChatRoom chat_room_id=id user_id=user_id />
                })}
            </div>
        </div>
    }
}
```

This layout creates a dashboard with a sidebar for the case list, a main content area for case details and document management, and a chat sidebar. You'll need to add logic to set the `selected_case_id` and `selected_chat_room_id` based on user interactions.

Remember to style your components and layout using CSS to create a polished and user-friendly interface. You may also want to add more interactivity, error handling, and loading states to improve the user experience.
