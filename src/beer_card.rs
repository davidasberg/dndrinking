use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(BeerCard)]
pub fn beer_card() -> Html {
    html! {
        <section class="py-10 bg-white sm:py-16 lg:py-24">
            <h1 class="text-4xl font-extrabold tracking-tight text-center text-gray-900 sm:text-5xl lg:text-6xl">{"Ölkort"}</h1>
        </section>
    }
}
