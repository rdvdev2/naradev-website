use domain::{project::Project, social::Social};
use hooks::use_data_asset::use_data_asset;
use yew::prelude::*;
use yew_router::prelude::*;

use components::{
    footer::Footer, header::Header, home::Home, not_found::NotFound, wip_banner::WIPBanner,
};

mod components;
mod domain;
mod hooks;

#[macro_export]
macro_rules! include_html {
    ($filename:literal) => {
        yew::prelude::Html::from_html_unchecked(yew::prelude::AttrValue::from(include_str!(
            $filename
        )))
    };
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/projects/:slug")]
    Project { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub const NAVIGATION: [Self; 2] = [Self::Home, Self::Projects];

    fn switch(self) -> Html {
        match self {
            Self::Home => html! {
                <Home />
            },
            Self::Projects => Self::NotFound.switch(),
            Self::Project { slug } => Self::NotFound.switch(),
            Self::NotFound => html! {
                <NotFound />
            },
        }
    }

    fn name(&self, projects: &Vec<Project>) -> String {
        match self {
            Self::Home => "Home",
            Self::Projects => "Projects",
            Self::Project { slug } => projects
                .iter()
                .filter(|x| x.get_slug() == slug)
                .next()
                .map_or("Not found", |x| x.get_name()),
            Self::NotFound => "Not found",
        }
        .to_owned()
    }
}

#[function_component]
fn TitleSetter() -> Html {
    let route = use_route::<Route>();
    let projects = use_context::<Vec<Project>>().unwrap();

    use_effect_with((route.clone(), projects.clone()), move |_| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!(
                "naritanara.xyz :: {}",
                route.unwrap_or_default().name(&projects)
            ));
    });

    html! {}
}

#[function_component]
fn App() -> Html {
    let projects = use_data_asset("projects.json");
    let socials = use_data_asset("socials.json");

    html! {
        <BrowserRouter>
            <ContextProvider<Vec<Project>> context={(*projects).clone()}>
            <ContextProvider<Vec<Social>> context={(*socials).clone()}>
                <TitleSetter />
                <div class="min-h-screen flex flex-col bg-gray-800 text-white">
                    <WIPBanner />
                    <Header/>
                        <Switch<Route> render={Route::switch} />
                    <Footer/>
                </div>
            </ContextProvider<Vec<Social>>>
            </ContextProvider<Vec<Project>>>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
