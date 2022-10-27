use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew::Properties;

#[derive(Deserialize, Clone)]
pub struct Flashcard {
    id: i32,
    question: String,
    answer: String,
    category: i32,
}

#[derive(Properties, PartialEq)]
pub struct CategoryProps {
    pub id: String,
}

#[function_component(Category)]
pub fn category(props: &CategoryProps) -> Html {
    let cards = use_state(|| {
        let cat: Vec<Flashcard> = vec![];
        cat
    });
    {
        let c = cards.clone();
        let id = props.id.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_cards: Vec<Flashcard> =
                        Request::get(&format!("/api/category/{}", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    c.set(fetched_cards);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <h3>{"Start learning with a card deck!"}</h3>
        </div>
    }
}
