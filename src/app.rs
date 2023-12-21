use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]

pub enum Route {
    #[at("/rules")]
    Rules,
    #[at("/")]
    Classes,
    #[at("/beercard")]
    BeerCard,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <div class="flex flex-col justify-center items-center m-4 gap-8">
                    <Nav />
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>

        </>
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

#[function_component(Classes)]
fn classes() -> Html {
    html! {
        <section class="grid lg:grid-cols-2 items-center gap-8 w-3/4">
            <ClassCard />
            <ClassCard />
            <ClassCard />
            <ClassCard />
        </section>
    }
}

#[function_component(BeerCard)]
fn beer_card() -> Html {
    html! {
        <section class="py-10 bg-white sm:py-16 lg:py-24">
            <h1 class="text-4xl font-extrabold tracking-tight text-center text-gray-900 sm:text-5xl lg:text-6xl">{"Ölkort"}</h1>
        </section>
    }
}

#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Rules));

    html! {
        <>
            <button {onclick}>{"Click to go home"}</button>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    let navigator = use_navigator().unwrap();

    let rules_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Rules));
        html! {
            <button {onclick} class="btn btn-primary">{"Rules"}</button>
        }
    };

    let classes_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Classes));
        html! {
            <button {onclick} class="btn btn-primary">{"Classes"}</button>
        }
    };

    let beer_card_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::BeerCard));
        html! {
            <button {onclick} class="btn btn-primary">{"Beer Card"}</button>
        }
    };

    html! {
        <nav class="bg-base-100 border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800">
            <div class="flex flex-wrap justify-between items-center mx-auto max-w-screen-xl">
                <div class="flex items-center lg:order-2 gap-4">
                    {rules_button}
                    {classes_button}
                    {beer_card_button}
                </div>
            </div>
        </nav>
    }
}

#[function_component(ClassCard)]
fn class_card() -> Html {
    html! {
        <div class="card-side flex flex-row shadow-xl bg-neutral">
            <figure><img src="https://daisyui.com/images/stock/photo-1494232410401-ad00d5433cfa.jpg" alt="Album"/></figure>
            <div class="card-body">
                <h2 class="card-title">{"Fighter"}</h2>
                <p>{"Fight your way through anything"}</p>
                <div class="card-actions justify-end">
                <button class="btn btn-primary">{"Read more"}</button>
                </div>
            </div>
        </div>
    }
}
