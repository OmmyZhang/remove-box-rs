use yew::{function_component, html, use_mut_ref, Html, Renderer};

use calc::Calc;
use game::Game;
use types::Role;

mod calc;
mod game;
mod types;

#[function_component(App)]
fn app() -> Html {
    let calc = use_mut_ref(|| Calc::new(7, 10));

    html! {
        <div>
            <Game
                players={[Role::AI, Role::AI]}
                width={6}
                difficulty={0.8}
                {calc}
            />
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
