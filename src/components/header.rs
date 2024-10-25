use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class="bg-purple-900 p-4 flex justify-between items-center drop-shadow-xl">
            <div class="flex gap-5">
                <Link<Route> to={Route::Home}>
                    <img src="assets/images/logo.png" class="h-8 rounded" />
                </Link<Route>>
                <Link<Route> to={Route::Home}>
                    <h1 class="font-bold text-2xl">{ "naritanara.xyz" }</h1>
                </Link<Route>>
            </div>
            <nav class="flex">
                { for Route::NAVIGATION.iter().map(|x| html! {
                    <div class="hover:underline m-1">
                    <Link<Route> to={x.clone()}>{ x.name() }</Link<Route>>
                    </div>
                })}
            </nav>
        </header>
    }
}
