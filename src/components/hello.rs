use yew::prelude::*;

use crate::components::card::Card;

#[function_component]
pub fn Hello() -> Html {
    html! {
        <Card>
            <img class="rounded-full w-20 border-2 border-purple-400" src="/assets/images/irl.jpg" alt="Profile image of me IRL" />
            <div class="text-justify max-w-prose">
                <h1 class="text-lg font-bold">{ "Hi, I'm Nara!" }</h1>
                <p>{ "I'm a hardware design enthusiast and an Informatics Engineering student at the UPC on Barcelona." }</p>
            </div>
        </Card>
    }
}
