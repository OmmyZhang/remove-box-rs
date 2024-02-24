use std::rc::Rc;

use web_sys::window;
use yew::{
    function_component, html, use_effect_with, use_memo, use_mut_ref, use_state, Callback, Html,
    Renderer,
};

use board::Board;
use calc::Calc;
use game::Game;
use implicit_clone::unsync::IString;
use modal::{Alert, Prompt};
use services::{get_record_list, upload_record};
use types::{Mode, Record, Role};

mod board;
mod calc;
mod cards;
mod game;
mod modal;
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

    let alert_text = use_state(|| None);

    let prompt_text = use_state(|| IString::from(""));
    let prompt_default = use_state(|| IString::from(""));
    let prompt_callback = use_state(|| None);

    {
        clone_all![record_list, show_record_list, alert_text];
        use_effect_with(*show_record_list, |&show| {
            if show && record_list.is_none() {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_record_list().await {
                        Ok(list) => record_list.set(Some(list)),
                        Err(msg) => {
                            alert_text.set(Some(IString::from(msg)));
                            show_record_list.set(false);
                        }
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
        clone_all![
            mode,
            level,
            record_list,
            alert_text,
            prompt_text,
            prompt_default,
            prompt_callback,
        ];
        Callback::from(move |winner: u8| {
            match *mode {
                Mode::Home => level.set((*level + 1) % 20),
                Mode::Pvp => mode.set(Mode::Home),
                Mode::Pve => {
                    if players[winner as usize] == Role::Local {
                        level.set(*level + 1);
                    } else {
                        let score = *level as i32 - 1;
                        mode.set(Mode::Home);
                        let last_record: Record = window()
                            .unwrap()
                            .local_storage()
                            .unwrap()
                            .unwrap()
                            .get_item("remove_box_last_record")
                            .unwrap()
                            .and_then(|last_record| serde_json::from_str(&last_record).ok())
                            .unwrap_or_default();

                        if score > last_record.score {
                            prompt_text
                                .set(IString::from(format!("New record! Level {}.", &score)));
                            prompt_default.set(IString::from(last_record.name.clone()));
                            prompt_callback.set({
                                clone_all![prompt_callback, record_list, alert_text];
                                Some(Callback::from(move |value| {
                                    if let Some(name) = value {
                                        gloo_console::log!(&name);

                                        let record = Record {
                                            name,
                                            score,
                                            time: None,
                                        };
                                        window()
                                            .unwrap()
                                            .local_storage()
                                            .unwrap()
                                            .unwrap()
                                            .set_item(
                                                "remove_box_last_record",
                                                &serde_json::to_string(&record).unwrap(),
                                            )
                                            .unwrap();

                                        clone_all![record_list, alert_text];
                                        wasm_bindgen_futures::spawn_local(async move {
                                            match upload_record(record).await {
                                                Ok(list) => record_list.set(Some(list)),
                                                Err(msg) => {
                                                    alert_text.set(Some(IString::from(msg)))
                                                }
                                            }
                                        })
                                    };
                                    prompt_callback.set(None);
                                }))
                            });
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

    let closed_alert = {
        clone_all![alert_text];
        Callback::from(move |()| alert_text.set(None))
    };

    html! {
        <>
            if let Some(text) = alert_text.as_ref() {
                <Alert {text} {closed_alert}/>
            }
            if let Some(callback) = prompt_callback.as_ref() {
                <Prompt
                    title={"Game Over"}
                    text={&*prompt_text}
                    default={&*prompt_default}
                    placeholder={"Your name"}
                    {callback}
                />
            }
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
