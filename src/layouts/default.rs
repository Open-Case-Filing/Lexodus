// use crate::components::{Footer, Nav};

use leptos::*;

use crate::components::{Footer, Nav, Action_Menu};

#[component]
pub fn Default_Layout(children: Children) -> impl IntoView {
    view! {
      

      <div class="bg-gray-900 bg-gradient-to-r from-cyan-800 to-blue-900">


      <div class="flex">
         <div class="flex flex-col min-h-screen text-white ">
         <div class="flex flex-1  divide-x divide-cyan-500 divide-dashed">
            <Nav/>
            <Action_Menu />
             {children()}
         </div>
         </div>
      </div>
      <Footer/>
      </div>    
    
    }   
}