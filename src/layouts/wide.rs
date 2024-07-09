// use crate::components::{Footer, Nav};

use leptos::*;

use crate::components::{Footer, Nav};

#[component]
pub fn Wide_Layout(children: Children) -> impl IntoView {
    view! {


      <div class="bg-gray-900 bg-gradient-to-r from-cyan-800 to-blue-900">


      <div class="flex">
         <div class="flex flex-col min-h-screen text-white ">
         <div class="flex flex-1">
            <Nav/>
            <div class="m-8 space-y-6 mr-2">
             {children()}
             </div>
         </div>
         </div>
      </div>
      <Footer/>
      </div>    
    
    }   
}