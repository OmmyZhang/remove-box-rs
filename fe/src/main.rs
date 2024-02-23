use std::rc::Rc;

use web_sys::window;
use yew::{
    function_component, html, use_effect_with, use_memo, use_mut_ref, use_state, Callback, Html,
    Renderer,
};

use board::Board;
use calc::Calc;
use game::Game;
use services::{get_record_list, upload_record};
use types::{Mode, Record, Role};

mod board;
mod calc;
mod cards;
mod game;
mod services;
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

    let record_list = use_state(|| None);
    let show_record_list = use_state(|| false);

    {
        clone_all![record_list];
        use_effect_with(*show_record_list, |&show| {
            if show && record_list.is_none() {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_record_list().await {
                        Ok(list) => record_list.set(Some(list)),
                        Err(msg) => window().unwrap().alert_with_message(&msg).unwrap(),
                    }
                });
            }
        });
    }

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
        clone_all![mode, level, record_list];
        Callback::from(move |winner: u8| {
            match *mode {
                Mode::Home => level.set((*level + 1) % 20),
                Mode::Pvp => mode.set(Mode::Home),
                Mode::Pve => {
                    if players[winner as usize] == Role::Local {
                        level.set(*level + 1);
                    } else {
                        let level: usize = *level;
                        mode.set(Mode::Home);
                        if level > 1 {
                            let user_name = window()
                                .unwrap()
                                .prompt_with_message(&format!("Level {}!\nYour name:", level - 1))
                                .unwrap();
                            let Some(user_name) = user_name
                                .map(|name| name.chars().take(30).collect::<String>())
                                .and_then(|name| (!name.is_empty()).then_some(name))
                            else {
                                return;
                            };

                            clone_all![record_list];
                            wasm_bindgen_futures::spawn_local(async move {
                                match upload_record(Record {
                                    name: user_name,
                                    score: level as i32 - 1,
                                    time: None,
                                })
                                .await
                                {
                                    Ok(list) => record_list.set(Some(list)),
                                    Err(msg) => window().unwrap().alert_with_message(&msg).unwrap(),
                                }
                            })
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
        <>
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
                        <button
                            onclick={{
                                clone_all![mode, level, n_hint, n_undo];
                                Callback::from(move |_| {
                                    mode.set(Mode::Pve);
                                    level.set(1);
                                    n_hint.set(N_HINT);
                                    n_undo.set(N_UNDO);
                                })
                            }}
                        >
                            { "Play vs AI" }
                        </button>
                        <button
                            onclick={{
                                clone_all![mode, level];
                                Callback::from(move |_| {
                                    mode.set(Mode::Pvp);
                                    level.set(WIDTH);
                                })
                            }}
                        >
                            { "Local Multiplayer" }
                        </button>
                        <button>{ "Online" }</button>
                        <button
                            onclick={{
                                clone_all![show_record_list];
                                Callback::from(move |_| show_record_list.set(!*show_record_list))
                            }}
                        >
                            { "Leaderboard" }
                        </button>
                    </div>
                    if *show_record_list {
                        <Board record_list={Rc::new((*record_list).clone())} />
                    }
                </div>
            }
        </>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
