use yew::prelude::*;
use yew_router::prelude::*;

use crate::beer_cards::BeerCards;
use crate::classes::Classes;
use crate::header::Header;
use crate::rules::Rules;

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
        <HashRouter>
        <div class="flex flex-col">
            <Header />
            <Switch<Route> render={switch} />
        </div>
        </HashRouter>
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
            <BeerCards />
        },
    }
}
