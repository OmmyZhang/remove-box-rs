use yew::{function_component, html, use_memo, use_mut_ref, use_state, Callback, Html, Renderer};

use calc::Calc;
use game::Game;
use gloo_console::log;
use types::{Mode, Role};

mod calc;
mod game;
mod types;

const WIDTH: usize = 7;
const HEIGHT: usize = 8;

macro_rules! clone_all {
    [$($s:ident), * $(,)?] => {
        $(
            let $s = $s.clone();
        )*
    };
}

pub(crate) use clone_all;

#[function_component(App)]
fn app() -> Html {
    let calc = use_mut_ref(|| Calc::new(WIDTH, HEIGHT));
    let level = use_state(|| 4);
    let mode = use_state(|| Mode::Home);
    let init_map = use_memo((*level, *mode), |(level, _)| {
        calc.borrow().gen_map(WIDTH.min(*level / 2 + 2))
    });

    let status = calc.borrow().encode_map(&init_map);
    let first_can_win = calc.borrow_mut().check_can_win(status);

    let players = match *mode {
        Mode::Home => [Role::AI, Role::AI],
        Mode::Pvp => [Role::Local, Role::Local],
        Mode::Pve => {
            if first_can_win {
                [Role::Local, Role::AI]
            } else {
                [Role::AI, Role::Local]
            }
        }
        // TODO
        Mode::Online => [Role::Local, Role::Remote],
    };

    let onend = {
        clone_all![mode, level];
        Callback::from(move |winner: u8| {
            match *mode {
                Mode::Home => level.set((*level + 1) % 20),
                Mode::Pvp => mode.set(Mode::Home),
                Mode::Pve => {
                    if winner == 0 {
                        level.set(*level + 1);
                    } else {
                        mode.set(Mode::Home);
                        // TODO
                    }
                }
                Mode::Online => {
                    mode.set(Mode::Home);
                    // TODO
                }
            }
        })
    };

    html! {
        <div>
            <Game
                key={status}
                {players}
                {init_map}
                {calc}
                {onend}
            />
            if *mode == Mode::Home {
                <div class="home-mask">
                    <div class="home-buttons">
                        <button onclick={{
                            clone_all![mode, level];
                            Callback::from(move |_| {
                                mode.set(Mode::Pve);
                                level.set(1);
                            })
                        }}>
                            { "Play vs AI" }
                        </button>
                        <button onclick={{
                            clone_all![mode, level];
                            Callback::from(move |_| {
                                mode.set(Mode::Pvp);
                                level.set(WIDTH);
                            })
                        }}>
                            { "Local Multiplayer" }
                        </button>
                        <button>{ "Online" }</button>
                        <button>{ "Leaderboard" }</button>
                    </div>
                </div>
            }
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
