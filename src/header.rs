use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let navigator = use_navigator().unwrap();

    let rules_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Rules));
        html! {
            <button {onclick} class="btn btn-primary normal-case text-xl transition delay-50 ease-in-out duration-150 hover:scale-110">{"Rules"}</button>
        }
    };

    let classes_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Classes));
        html! {
            <button {onclick} class="btn btn-primary normal-case text-xl transition delay-50 ease-in-out duration-150 hover:scale-110">{"Classes"}</button>
        }
    };

    let beer_card_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::BeerCard));
        html! {
            <button {onclick} class="btn btn-primary normal-case text-xl transition delay-50 ease-in-out duration-150 hover:scale-110">{"Beer Card"}</button>
        }
    };

    html! {
        <div class="navbar bg-base-100">
            <div class="navbar-start"/>

            <div class="navbar-center flex gap-4">
                {rules_button}
                {classes_button}
                {beer_card_button}
            </div>

            <div class="navbar-end"/>
        </div>
    }
}
