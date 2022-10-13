use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Default, Clone, PartialEq)]
struct User {
    id: u64,
    username: String,
}

#[function_component(FetchExample)]
pub fn fetch_example() -> Html {
    let user_from_server = use_state(|| User::default());
    {
        let user = user_from_server.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_user: User = Request::get("/api/user")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    user.set(fetched_user);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <span>{ "Hello to " }{(*user_from_server).clone().username}</span>
        </>
    }
}
