use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <main class="flex-grow flex flex-col items-center">
            <h1 class="text-8xl p-5">{ "404" }</h1>
            <h2 class="text-xl">{ "Not found" }</h2>
            <div class="m-5 underline hover:font-semibold">
                <Link<Route> to={Route::Home}>{ "Go back to home" }</Link<Route>>
            </div>
        </main>
    }
}
