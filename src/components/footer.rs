use chrono::Datelike;
use yew::prelude::*;

use crate::{components::social_image_link::SocialImageLink, domain::social::Social};

#[function_component]
pub fn Footer() -> Html {
    let socials = use_context::<Vec<Social>>().unwrap();

    let year = chrono::Local::now().year();

    html! {
        <footer class="bg-gray-700 flex flex-col justify-center items-center p-1 text-sm shadow-xl">
        <div class="flex my-2 gap-4">
            { for socials.iter().cloned().map(|x| html! { <><SocialImageLink social={x} /></> }) }
        </div>
        {format!("© {year}, Nara Díaz Viñolas. All rights reserved.")}
        </footer>
    }
}
