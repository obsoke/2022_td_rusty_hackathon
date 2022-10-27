mod components;

use crate::components::category::Category;
use crate::components::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/category/:id")]
    Category { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::HomePage => html! {
            <Home />
        },
        Route::Category { id } => html! {
            <Category id={id.to_owned()} />
        },
        Route::NotFound => html! { <h3>{ "Page not found" }</h3> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <h1 class="h-liners">{ "Flashcarder" }</h1>
            <h2>{ "Learn some things!" }</h2>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
