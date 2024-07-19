use leptos::*;
use leptos_router::ActionForm;
use crate::layouts::wide::Wide_Layout;
use crate::models::case::Case;


// Define the structure for our case data
#[server(AddCase, "/api")]
pub async fn add_case(case_number: String, case_title: String) -> Result<Vec<Case>, ServerFnError> {
    // Add the case to the database
    // This is where you would add the case to your database
    // For now, we'll just print it to the console
    use chrono::{TimeZone, Utc};
    use leptos_spin::redirect;
    let add_case = create_server_action::<AddCase>();
    println!("Adding case: {} {}", case_number, case_title);

    let has_error = move || add_case.value().with(|val| matches!(val, Some(Err(_))));
    if has_error() {
        return Err(ServerFnError::new("Error adding case"));
    }
    redirect("/dashboard/overview");

    let add_civil_case = vec![
        Case {
            id: "15".into(),
            case_name: case_title.into(),
            case_number: case_number.into(),
            court: "Eastern District of Michigan".into(),
            date_filed: Utc.with_ymd_and_hms(2022, 5, 5, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 3, 10, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Consumer Protection".into()),
            jurisdiction_type: "Diversity".into(),
            assigned_to: Some("Judge Lee".into()),
            cause: Some("Product Liability".into()),
            docket_number: "7056".into(),
            status: "Open".into(),
        },
    ];

  
    Ok(add_civil_case)
}


#[component]
pub fn CivilCaseForm() -> impl IntoView {
    let add_case = create_server_action::<AddCase>();
    let value = add_case.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    
        view! {
            <Wide_Layout>
            <div class="flex justify-center items-center min-h-screen bg-gray-800 text-gray-400 p-6">
            <div class="bg-gray-800 p-8 rounded-lg shadow-lg w-full max-w-4xl">
                <h2 class="text-2xl font-semibold mb-6 text-center text-cyan-500 uppercase tracking-wider">"Open a New Federal Civil Case"</h2>
                <ActionForm
                    action=add_case
                    class="space-y-6"
                >
                    // Case Information
                    <fieldset class="border border-gray-700 p-4 rounded-md">
                    <legend class="text-sm font-semibold text-cyan-500 uppercase tracking-wider px-2">"Case Information"</legend>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="w-full">
                        <InputField name="case_number" label="Case Number" required=true />
                        </div>
                        <InputField name="case_title" label="Case Title" required=true />
                        <SelectField name="division" label="Division" required=true>
                            <option value="">"Select Division"</option>
                            <option value="central">"Central Division"</option>
                            <option value="eastern">"Eastern Division"</option>
                            <option value="western">"Western Division"</option>
                        </SelectField>
                        <SelectField name="case_type" label="Case Type" required=true>
                            <option value="">"Select Case Type"</option>
                            <option value="civil_rights">"Civil Rights"</option>
                            <option value="contract">"Contract"</option>
                            <option value="real_property">"Real Property"</option>
                            <option value="tort">"Tort"</option>
                            <option value="labor">"Labor"</option>
                            <option value="immigration">"Immigration"</option>
                            <option value="prisoner">"Prisoner Petition"</option>
                            <option value="forfeiture">"Forfeiture/Penalty"</option>
                            <option value="bankruptcy">"Bankruptcy"</option>
                            <option value="ip">"Intellectual Property"</option>
                            <option value="social_security">"Social Security"</option>
                            <option value="tax">"Tax"</option>
                            <option value="other">"Other"</option>
                        </SelectField>
                        <InputField name="filing_date" label="Filing Date" type_="date" required=true />
                        <InputField name="jury_demand" label="Jury Demand" type_="checkbox" />
                        <InputField name="demand" label="Demand ($)" type_="number" />
                        <InputField name="related_cases" label="Related Case(s)" />
                    </div>
                </fieldset>
    
                        // Plaintiff Information
                        <fieldset class="border border-gray-700 p-4 rounded-md">
                        <legend class="text-sm font-semibold text-cyan-500 uppercase tracking-wider px-2">"Case Information"</legend>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <InputField name="plaintiff_name" label="Plaintiff Name" required=true />
                                <InputField name="plaintiff_address" label="Plaintiff Address" required=true />
                                <InputField name="plaintiff_phone" label="Plaintiff Phone" type_="tel" />
                                <InputField name="plaintiff_email" label="Plaintiff Email" type_="email" />
                            </div>
                        </fieldset>
    
                        // Defendant Information
                        <fieldset class="border border-gray-700 p-4 rounded-md">
                        <legend class="text-sm font-semibold text-cyan-500 uppercase tracking-wider px-2">"Case Information"</legend>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <InputField name="defendant_name" label="Defendant Name" required=true />
                                <InputField name="defendant_address" label="Defendant Address" required=true />
                                <InputField name="defendant_phone" label="Defendant Phone" type_="tel" />
                                <InputField name="defendant_email" label="Defendant Email" type_="email" />
                            </div>
                        </fieldset>
    
                        // Attorney Information
                        <fieldset class="border border-gray-700 p-4 rounded-md">
                        <legend class="text-sm font-semibold text-cyan-500 uppercase tracking-wider px-2">"Case Information"</legend>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <InputField name="attorney_name" label="Attorney Name" required=true />
                                <InputField name="attorney_firm" label="Law Firm" />
                                <InputField name="attorney_bar_number" label="Bar Number" required=true />
                                <InputField name="attorney_phone" label="Phone" type_="tel" required=true />
                                <InputField name="attorney_email" label="Email" type_="email" required=true />
                            </div>
                        </fieldset>
    
                        // Case Details
                        <fieldset class="border border-gray-700 p-4 rounded-md">
                        <legend class="text-sm font-semibold text-cyan-500 uppercase tracking-wider px-2">"Case Information"</legend>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <TextareaField name="nature_of_suit" label="Nature of Suit" required=true />
                                <TextareaField name="cause_of_action" label="Cause of Action" required=true />
                                <TextareaField name="relief_sought" label="Relief Sought" required=true />
                            </div>
                        </fieldset>
    
                        <div>
                        <input
                            type="submit"
                            value="File Case"
                            class="w-full px-4 py-2 bg-cyan-600 hover:bg-cyan-700 text-white rounded-md font-medium cursor-pointer transition duration-200"
                        />
                    </div>
    
                    {move || if has_error() {
                        view! {
                            <p class="mt-2 text-sm text-red-400">
                                "Error filing case. Please check your inputs and try again."
                            </p>
                        }
                    } else {
                        view! { <p></p> }
                    }}
                </ActionForm>
            </div>
            </div>
            </Wide_Layout>
        }
    }
    
    #[component]
    fn InputField(
        name: &'static str,
        label: &'static str,
        #[prop(optional)] required: bool,
        #[prop(optional)] type_: &'static str,
    ) -> impl IntoView {
        let input_type = if type_.is_empty() { "text" } else { type_ };
        view! {
            <div>
                <label for={name} class="block text-sm font-medium mb-1 text-cyan-500">
                    {label}
                </label>
                <input 
                    type={input_type}
                    id={name}
                    name={name}
                    required={required}
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 text-white"
                />
            </div>
        }
    }
    
    #[component]
    fn SelectField(
        name: &'static str,
        label: &'static str,
        required: bool,
        children: Children,
    ) -> impl IntoView {
        view! {
            <div>
                <label for={name} class="block text-sm font-medium mb-1 text-cyan-500">
                    {label}
                </label>
                <select
                    id={name}
                    name={name}
                    required={required}
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 text-white"
                >
                    {children()}
                </select>
            </div>
        }
    }
    
    #[component]
    fn TextareaField(
        name: &'static str,
        label: &'static str,
        required: bool,
    ) -> impl IntoView {
        view! {
            <div>
                <label for={name} class="block text-sm font-medium mb-1 text-cyan-500">
                    {label}
                </label>
                <textarea
                    id={name}
                    name={name}
                    required={required}
                    rows="3"
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 text-white"
                ></textarea>
            </div>
        }
    }