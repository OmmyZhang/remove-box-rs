use web_sys::window;
use yew::{function_component, html, use_memo, use_mut_ref, use_state, Callback, Html, Renderer};

use calc::Calc;
use game::Game;
use gloo_console::log;
use types::{Mode, Role};

mod calc;
mod cards;
mod game;
mod types;

const WIDTH: usize = 7;
const HEIGHT: usize = 8;

const N_HINT: u32 = 10;
const N_UNDO: u32 = 20;

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

    let n_hint = use_state(|| 0);
    let n_undo = use_state(|| 0);

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
                    if players[winner as usize] == Role::Local {
                        level.set(*level + 1);
                    } else {
                        mode.set(Mode::Home);
                        if *level > 1 {
                            let user_name = window()
                                .unwrap()
                                .prompt_with_message(&format!("Level {}!\nYour name:", *level - 1))
                                .unwrap();
                            log!(user_name);
                        }
                    }
                }
                Mode::Online => {
                    mode.set(Mode::Home);
                    // TODO
                }
            }
        })
    };

    let use_hint = {
        clone_all![n_hint];
        Callback::from(move |()| n_hint.set(*n_hint - 1))
    };

    let use_undo = {
        clone_all![n_undo];
        Callback::from(move |()| n_undo.set(*n_undo - 1))
    };

    html! {
        <div>
            <Game
                key={status}
                {players}
                {init_map}
                {calc}
                {onend}
                mode={*mode}
                level={*level}
                n_hint={*n_hint}
                n_undo={*n_undo}
                {use_hint}
                {use_undo}
            />
            if *mode == Mode::Home {
                <div class="home-mask">
                    <div class="home-buttons">
                        <button onclick={{
                            clone_all![mode, level, n_hint, n_undo];
                            Callback::from(move |_| {
                                mode.set(Mode::Pve);
                                level.set(1);
                                n_hint.set(N_HINT);
                                n_undo.set(N_UNDO);
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
