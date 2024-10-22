use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    let Props { children, class } = props;

    html! {
        <div class={classes!(
            class.clone(),
            "bg-gray-600", "p-4", "rounded-2xl", "flex", "gap-4",
            "justify-center", "items-center", "w-fit", "shadow-2xl"
        )}>
            { children }
        </div>
    }
}
