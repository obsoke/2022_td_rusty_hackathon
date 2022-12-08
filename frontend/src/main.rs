mod components;

use crate::components::category::Category;
use crate::components::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/category/:id")]
    Category { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {
            <Home />
        },
        Route::Category { id } => html! {
            <Category id={id} />
        },
        Route::NotFound => html! { <h3>{ "Page not found" }</h3> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <h1 class="h-liners"><Link<Route> to={Route::HomePage}>{ "Flashcarder" }</Link<Route>></h1>
            <h2>{ "Learn some things!" }</h2>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
