mod app;
mod beer_cards;
mod classes;
mod header;
mod rules;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
