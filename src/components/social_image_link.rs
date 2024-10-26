use yew::prelude::*;

use crate::domain::social::Social;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub social: Social,
}

#[function_component]
pub fn SocialImageLink(props: &Props) -> Html {
    let Props { social } = props;

    let href = social.get_link().to_owned();
    let img_src = social.get_logo_url();
    let img_alt = format!("An image link to {}", social.get_name());

    html! {
        <a href={href} target="_blank" rel="noopener noreferrer" class="w-fit h-fit aspect-square">
            <img src={img_src} alt={img_alt} class="block m-auto h-10 aspect-square drop-shadow-2xl" />
        </a>
    }
}
