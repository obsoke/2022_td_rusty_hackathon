mod components;

use crate::components::counter::Counter;
use crate::components::fetch_ex::FetchExample;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h2>{"Home"}</h2>
                <Counter />
                <FetchExample />
            </>
        },
        Route::NotFound => html! { <h2>{" 404" }</h2> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <h1>{ "Cool App" }</h1>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
