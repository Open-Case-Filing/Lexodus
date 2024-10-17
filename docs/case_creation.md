# Case Creation Flow in Lexodus

## 1. User Navigation

The user starts by navigating to the Case Management page. This is typically done through a menu item or a link in the application's navigation bar.

```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                // ... other navigation items
                <A href="/cases">"Case Management"</A>
            </nav>

            <Routes>
                // ... other routes
                <Route path="/cases" view=CaseManagement/>
            </Routes>
        </Router>
    }
}
```

## 2. Case Creation Form

Once on the Case Management page, the user sees the `CreateCaseForm` component, which is part of the `CaseManagement` component.

```rust
#[component]
pub fn CaseManagement() -> impl IntoView {
    view! {
        // ... other elements
        <CreateCaseForm />
        // ... CaseList component
    }
}
```

The `CreateCaseForm` component renders a form with input fields for case details.

## 3. Form Submission

When the user fills out the form and clicks the "Create Case" button, the form submission is handled by Leptos' `ActionForm` component:

```rust
#[component]
pub fn CreateCaseForm() -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();

    view! {
        <ActionForm action=create_case>
            // ... form fields
            <button type="submit">"Create Case"</button>
        </ActionForm>
    }
}
```

## 4. Server-side Processing

The form submission triggers the `CreateCase` server function:

```rust
#[server(CreateCase, "/api")]
pub async fn create_case(
    case_number: String,
    title: String,
    status: String,
    filed_date: String,
    court_id: i64,
    judge_id: Option<i64>,
) -> Result<String, ServerFnError> {
    // Database interaction code
    // ...
}
```

This function interacts with the database to insert the new case.

## 5. Response Handling

After the server function completes, Leptos automatically updates the `create_case.value()` signal with the result.

## 6. User Feedback

The `CreateCaseForm` component listens to this signal and provides feedback to the user:

```rust
#[component]
pub fn CreateCaseForm() -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let response = create_case.value();

    view! {
        // ... form elements
        <Show
            when=move || create_case.pending().get()
            fallback=|| view! { <div></div> }
        >
            <div>"Creating case..."</div>
        </Show>

        {move || response.get().map(|result| match result {
            Ok(message) => view! { <div class="text-green-500">{message}</div> },
            Err(e) => view! { <div class="text-red-500">{e.to_string()}</div> },
        })}
    }
}
```

## 7. List Update

After a successful case creation, you'll want to update the list of cases. This can be done by triggering a refetch of the cases data:

```rust
#[component]
pub fn CaseManagement() -> impl IntoView {
    let cases_resource = create_resource(
        || (),
        |_| get_cases()
    );

    let refresh_cases = move |_| {
        cases_resource.refetch();
    };

    view! {
        <CreateCaseForm on_success=refresh_cases />
        <CaseList cases=cases_resource />
    }
}
```

In this setup, we pass a `refresh_cases` callback to the `CreateCaseForm`. The form would call this callback upon successful case creation:

```rust
#[component]
pub fn CreateCaseForm(on_success: Callback<()>) -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let response = create_case.value();

    create_effect(move |_| {
        if let Some(Ok(_)) = response.get() {
            on_success.call(());
        }
    });

    // ... rest of the component
}
```

This approach ensures that the case list is updated immediately after a new case is successfully created.
