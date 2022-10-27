use crate::Route;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

// Sharing between types crate was not thought out well enough...l
#[derive(Deserialize, Clone)]
pub struct Category {
    id: i32,
    name: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let categories = use_state(|| {
        let cat: Vec<Category> = vec![];
        cat
    });
    {
        let cats = categories.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_categories: Vec<Category> = Request::get("/api/category")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    cats.set(fetched_categories);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <h3>{"Start learning with a card deck!"}</h3>
            {
                if categories.len() == 0 {
                    html! { "No categories created" }
                } else {
                    html! {
                    <ul>
                    {
                        categories.iter().map(|category| {
                            html!{<li key={category.id}><Link<Route> to={Route::Category { id: category.id.to_string() }}>{ &category.name }</Link<Route>></li>}
                        }).collect::<Html>()
                    }
                    </ul>
                    }
                }
            }
        </div>
    }
}
