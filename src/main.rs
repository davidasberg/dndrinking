mod app;
mod beer_card;
mod classes;
mod header;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
