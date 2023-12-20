use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex flex-row min-h-screen justify-center items-center m-4">
        <div class="flex flex-col items-center gap-8 w-3/4">
            <ClassCard />
            <ClassCard />
            <ClassCard />
            <ClassCard />
        </div>
        </div>
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
