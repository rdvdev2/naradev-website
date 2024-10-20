use yew::prelude::*;
use yew_router::prelude::*;

use components::{footer::Footer, header::Header, wip_banner::WIPBanner};

mod components;
mod domain;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub const NAVIGATION: [Self; 1] = [Self::Home];

    fn switch(self) -> Html {
        match self {
            Self::Home => html! {
                { "TODO: This page" }
            },
            Self::NotFound => html! {
                { "404: Not found" }
            },
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Home => "Home",
            Self::NotFound => "Not found",
        }
        .to_owned()
    }
}

#[function_component]
fn TitleSetter() -> Html {
    let route = use_route::<Route>();
    use_effect_with(route.clone(), |_| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("Nara DEV :: {}", route.unwrap_or_default().name()));
    });

    html! {}
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <TitleSetter />
            <div class="min-h-screen flex flex-col bg-gray-800 text-white">
                <WIPBanner />
                <Header/>
                <main class="flex-grow">
                    <Switch<Route> render={Route::switch} />
                </main>
                <Footer/>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
