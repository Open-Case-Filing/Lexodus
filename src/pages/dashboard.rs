use leptos::*;
use leptos_meta::*;
use crate::layouts::default::*;
// use crate::components::search_bar::*;


#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Dashboard | Open Case Filing System"/>
        <Title text="Dashboard | Open Case Filing System"/>
        <Meta name="description" content="Dashboard overview for OCFS with real time case numbers for the week, month, year."/>
        <Meta property="og:description" content="A dashboard with case management statistics and ability to see civil and criminal case information."/>
        <Meta
          property="og:image"
          content="https://en.wikipedia.org/wiki/CM/ECF#/media/File:CM_ECF_logo.png"
        />
        <Default_Layout>

                        <h2 class="text-2xl font-semibold">Welcome to Lexodus</h2>

        </Default_Layout>
    }
}
