use crate::components::card::Card;
use crate::components::project_card::ProjectCard;
use crate::domain::project::Project;
use crate::Route;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn FeaturedProjects() -> Html {
    let projects = use_state(|| vec![]);
    {
        let projects = projects.clone();
        use_effect_with((), move |_| {
            let projects = projects.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_projects: Vec<Project> = Request::get("/assets/data/projects.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                projects.set(fetched_projects);
            });
            || ()
        });
    }

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
