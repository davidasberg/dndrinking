use yew::prelude::*;
use yew_router::prelude::*;

use crate::header::Header;

#[derive(Clone, Routable, PartialEq)]

pub enum Route {
    #[at("/")]
    Rules,
    #[at("/classes")]
    Classes,
    #[at("/beercard")]
    BeerCard,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <div class="flex flex-col">
            <Header />
            <Switch<Route> render={switch} />
        </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Rules => html! {
            <Rules />
        },
        Route::Classes => html! {
            <Classes />
        },
        Route::BeerCard => html! {
            <BeerCard />
        },
    }
}

#[function_component(Rules)]
fn rules() -> Html {
    html! {
        <section class="py-10 bg-white sm:py-16 lg:py-24">
            <h1 class="text-4xl font-extrabold tracking-tight text-center text-gray-900 sm:text-5xl lg:text-6xl">{"Regler"}</h1>
        </section>
    }
}
