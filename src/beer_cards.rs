use serde::Deserialize;
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

use rand::seq::SliceRandom;

#[derive(Clone, Copy, Eq, PartialEq, Deserialize)]
struct Card {
    title: &'static str,
    description: &'static str,
    probability: u32,
}

#[function_component(BeerCards)]
pub fn beer_cards() -> Html {
    let cards = include_str!("../data/beer_cards.ron");
    let mut cards: Vec<Card> = ron::from_str(cards).unwrap();

    let current_card = use_state(|| cards[0]);

    cards.sort_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap());

    let onclick = {
        let card = current_card.clone();
        Callback::from(move |_| {
            while let Ok(&new_card) =
                cards.choose_weighted(&mut rand::thread_rng(), |card| card.probability)
            {
                if new_card != *card {
                    card.set(new_card);
                    break;
                }
            }
        })
    };

    html! {
        <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-10 font-extrabold tracking-tight text-center sm:text-5xl lg:text-6xl">{"Beer Cards"}</h1>
            <p1 class="text-lg text-center">{"Ölkort är roliga! Dra ett kort när du slår en 20a"}</p1>

            <button onclick={onclick} class="btn btn-primary normal-case text-xl my-6 transition delay-50 ease-in-out duration-150 hover:scale-110">{"Dra ett kort"}</button>

            <BeerCard title={(*current_card).title} description={(*current_card).description} />

        </div>
    }
}

#[function_component(BeerCard)]
fn beer_card(props: &BeerCardProps) -> Html {
    let BeerCardProps { title, description } = props;

    html! {
        <div class="card w-96 bg-base-100 shadow-xl image-full transition delay-50 ease-in-out duration-150 hover:scale-110">
        <div class="card-body">
          <h2 class="card-title">{title}</h2>
          <p>{description}</p>
        </div>
      </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct BeerCardProps {
    title: AttrValue,
    description: AttrValue,
}
