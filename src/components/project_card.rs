use yew::prelude::*;
use yew_router::prelude::*;

use crate::{domain::project::Project, Route};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub project: Project,
}

#[function_component]
pub fn ProjectCard(props: &Props) -> Html {
    let Props { project } = props;

    html! {
        <Link<Route> to={Route::Project { slug: project.get_slug().to_owned() }}>
            <div class="bg-black overflow-hidden aspect-square max-w-xs rounded-3xl border-purple-500 border-2 transition-all hover:scale-105 cursor-pointer">
                <div
                    id="project_card_inner"
                    style={format!("--bg-url: url('{}')", project.get_card_background_image())}
                    class="grid grid-cols-1 grid-rows-2 items-center p-4 w-full h-full transition-all opacity-70 hover:opacity-100"
                >
                    <h1 class="text-xl font-bold text-center">{ project.get_name() }</h1>
                    <p class="text-justify">{ project.get_short_description() }</p>
                </div>
            </div>
        </Link<Route>>
    }
}
