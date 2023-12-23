use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

use rand::seq::SliceRandom;

const CARDS: [Card; 3] = [
    Card {
        title: "Gudarna har bestämt att du ska byta klass",
        description: "Rulla en tärning, vid 20 får du byta till DM, annars får du välja en klass att byta till",
        background_image: None,
    },
    Card {
        title: "Oscar fattar inte",
        description: "Oscar förstår inte reglerna, han dricker 2 klunkar",
        background_image: None,
    },
    Card {
        title: "Drick öl",
        description: "Drick en öl",
        background_image: None,
    },
];

struct Card {
    title: &'static str,
    description: &'static str,
    background_image: Option<&'static str>,
}

#[function_component(BeerCards)]
pub fn beer_cards() -> Html {
    let current_card = CARDS.choose(&mut rand::thread_rng());

    html! {
        <div class="container mx-auto flex flex-col justify-center items-center">
            <h1 class="text-4xl my-10 font-extrabold tracking-tight text-center sm:text-5xl lg:text-6xl">{"Beer Cards"}</h1>
            <p1 class="text-lg text-center">{"Ölkort är roliga! Dra ett kort när du slår en 20a"}</p1>

            <button class="btn btn-primary normal-case text-xl my-6">{"Dra ett kort"}</button>

            if let Some(card) = current_card {
                <BeerCard title={card.title} description={card.description} background_image={card.background_image} />
            }
        </div>
    }
}

#[function_component(BeerCard)]
fn beer_card(props: &BeerCardProps) -> Html {
    let BeerCardProps {
        title,
        description,
        background_image,
    } = props;

    html! {
        <div class="card w-96 bg-base-100 shadow-xl image-full">
        <figure><img src={background_image} alt="beer_card" /></figure>
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
    background_image: Option<AttrValue>,
}
