// use crate::components::{Footer, Nav};

use leptos::*;

use crate::components::{Footer, Nav, Action_Menu};

#[component]
pub fn Default_Layout(children: Children) -> impl IntoView {
    view! {
      <div class="h-screen flex bg-gray-900 text-white">
          // Left column: Global Navigation (75px wide)
          <nav class="w-[64] bg-gray-800 p-4  flex-shrink-1">

                 <Nav/>
              </nav>

              // Right column: Action Bar and Main Content

                  // Action Bar
                  <Action_Menu />

                  // Main Content
                  <main class="flex-1 overflow-y-auto p-6 flex justify-center">
                      <div class="w-full max-w-4xl">

                      {children()}
                      </div>

                  </main>


      </div>

      <Footer/>


    }
}
