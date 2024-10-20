use yew::prelude::*;

#[function_component]
pub fn WIPBanner() -> Html {
    let hidden = use_state(|| false);
    let onclick = {
        let hidden = hidden.clone();
        move |_| {
            hidden.set(true);
        }
    };

    html! {
        <div class={classes!(
            "flex", "flex-row", "justify-center", "bg-yellow-500", "transition-all", "gap-1",
            (!*hidden).then_some("p-1.5"),
            hidden.then_some("p-0"),
            hidden.then_some("h-0")
        )}>
            <h1 class="text-black italic text-sm">{ "🚧 This website is a Work In Progress! 🚧" }</h1>
            <button class="underline text-black text-xs" type="button" {onclick}>{ "Hide" }</button>
        </div>
    }
}
