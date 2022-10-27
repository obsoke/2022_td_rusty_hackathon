use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew::{Callback, Properties};

#[derive(Deserialize, Clone, Debug)]
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
    let counter = use_state(|| 0);
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
                    let fetched_cards: Vec<Flashcard> = Request::get(&format!("/api/card/{}", id))
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

    let onclick = {
        let counter = counter.clone();
        let cards = cards.clone();
        Callback::from(move |_| {
            let num_cards = cards.len();
            if num_cards - 1 == *counter {
                counter.set(0)
            } else {
                counter.set(*counter + 1)
            }
        })
    };

    html! {
        <div>
            <h3>{"Start learning with a card deck!"}</h3>
            {
                match cards.get(*counter) {
                    Some(card) => html! {
                        <div class="card">
                            <div class="question"> {&card.question}</div>
                            <div class="answer"> {&card.answer}</div>
                        </div>
                    },
                    None => html!{},
                }
            }

        <div class="deck-btns">
            <button {onclick}> { "Next!" }</button>
         </div>
      </div>
    }
}
