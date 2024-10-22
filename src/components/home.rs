use yew::prelude::*;

use crate::components::{about_me::AboutMe, hello::Hello};

#[function_component]
pub fn Home() -> Html {
    html! {
        <main class="flex-grow flex flex-col p-5 items-center gap-5">
            <Hello />
            <AboutMe />
        </main>
    }
}
