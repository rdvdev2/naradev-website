use yew::prelude::*;

use crate::{components::card::Card, include_html};

#[function_component]
pub fn AboutMe() -> Html {
    html! {
        <Card class="flex-wrap">
            <img class="rounded-xl max-w-sm" src="https://images.unsplash.com/photo-1517694712202-14dd9538aa97?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=870&q=80" alt="Placeholder image of a computer" />
            <p class="text-justify basis-3/5 flex-grow">
                {include_html!("../content/about_me.html")}
            </p>
        </Card>
    }
}
