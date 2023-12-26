use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

use rand::seq::SliceRandom;

const CARDS: [Card; 4] = [
    Card {
        title: "Ölkort",
        description: "Det här är ett ölkort, dra ett ölkort när du slår 20!",
    },
    Card {
        title: "Gudarna har bestämt att du ska byta klass",
        description: "Rulla en tärning, vid 20 får du byta till DM, annars får du välja en klass att byta till",
    },
    Card {
        title: "Oscar fattar inte",
        description: "Oscar förstår inte reglerna, han dricker 2 klunkar",
    },
    Card {
        title: "Drick öl",
        description: "Drick en öl",
    },
];

#[derive(Clone, Copy, Eq, PartialEq)]
struct Card {
    title: &'static str,
    description: &'static str,
}

#[function_component(BeerCards)]
pub fn beer_cards() -> Html {
    let current_card = use_state(|| CARDS[0]);

    let onclick = {
        let card = current_card.clone();
        Callback::from(move |_| {
            while let Some(&new_card) = CARDS[1..].choose(&mut rand::thread_rng()) {
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
