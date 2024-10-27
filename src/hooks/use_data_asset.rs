use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use yew::prelude::*;

#[hook]
pub fn use_data_asset<T: DeserializeOwned + 'static>(filename: &str) -> UseStateHandle<Vec<T>> {
    let filename = filename.to_owned();
    let data = use_state(|| vec![]);
    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Vec<T> = Request::get(&format!("/assets/data/{filename}"))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });
            || ()
        });
    }

    data
}
