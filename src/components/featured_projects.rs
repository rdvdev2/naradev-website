use crate::components::card::Card;
use crate::components::project_card::ProjectCard;
use crate::domain::project::Project;
use crate::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn FeaturedProjects() -> Html {
    let projects = use_context::<Vec<Project>>().unwrap();

    html! {
        <Card class="flex-col">
            <h2 class="text-2xl font-bold">{ "Featured projects" }</h2>
            <p class="text-justify">{ "In this section you'll find my latest and greatest projects, personally selected by me! If you're interested in some of my less important projects, you can " }<Link<Route> to={Route::Projects}><span class="underline">{ "check them out here" }</span></Link<Route>>{" too." }</p>
            <div class="flex flex-wrap gap-3 justify-center">
                { for projects.iter().cloned().map(|x| html! { <ProjectCard project={x} /> } ) }
            </div>
        </Card>
    }
}
