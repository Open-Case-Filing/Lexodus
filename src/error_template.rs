use crate::errors::LexodusAppError;
use cfg_if::cfg_if;
use leptos::{Errors, *};

#[cfg(feature = "ssr")]
use leptos_spin::ResponseOptions;

// A basic function to display errors served by the error boundaries. Feel free to do more complicated things
// here than just displaying them
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };

    // Get Errors from Signal
    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<LexodusAppError> = errors
        .get()
        .into_iter()
        .filter_map(|(_, v)| v.downcast_ref::<LexodusAppError>().cloned())
        .collect();

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! {
      if #[cfg(feature="ssr")]{
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response{
          response.set_status(errors[0].status_code().into());
        }
      }
    }

    view! {
      <h1>"Errors"</h1>
      <For
        each=move || { errors.clone().into_iter().enumerate() }
        key=|(index, _error)| *index
        children=move |error| {
            let error_string = error.1.to_string();
            let error_code = error.1.status_code();
            view! {
              <h2>{error_code.to_string()}</h2>
              <p>"Error: " {error_string}</p>
            }
        }
      />
    }
}
