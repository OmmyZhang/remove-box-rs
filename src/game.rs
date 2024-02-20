use std::cell::RefCell;
use std::rc::Rc;

use gloo_console::log;
use gloo_timers::callback::Timeout;
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html};
use yew_autoprops::autoprops;

use crate::calc::Calc;
use crate::clone_all;
use crate::types::{Role, Stage};

#[autoprops]
#[function_component(Game)]
pub fn game(
    players: &[Role; 2],
    init_map: Rc<Vec<usize>>,
    calc: Rc<RefCell<Calc>>,
    onend: Callback<u8>,
) -> Html {
    let curr_player = use_state(|| 0_u8);
    let curr_stage = use_state(|| Stage::Waiting);
    let map = use_state(|| (*init_map).clone());

    let pressed1 = use_state(|| None);
    let pressed2 = use_state(|| None);

    let chosen_range = use_state(|| None);
    let chosen_state = use_state(|| "");
    // (start_i, end_i, j)
    let last_remove_line = use_state(|| None);

    let winner = use_state(|| None);

    // 点击格子
    let onclick = {
        clone_all![
            players,
            curr_player,
            curr_stage,
            map,
            pressed1,
            pressed2,
            chosen_range,
            chosen_state,
            last_remove_line,
            winner,
        ];
        Callback::from(
            move |(i, j, i2j2): (usize, usize, Option<(usize, usize)>)| {
                log!(i, j, format!("{:?}, stage: {:?}", i2j2, *curr_stage));

                if i2j2.is_none() && players[*curr_player as usize] != Role::Local {
                    return;
                }

                if *curr_stage == Stage::Waiting || i2j2.is_some() {
                    pressed1.set(Some((i, j)));
                    curr_stage.set(Stage::Press1);
                    last_remove_line.set(None);
                }

                let (i1j1, i2j2) = if *curr_stage == Stage::Press1 {
                    (pressed1.unwrap(), Some((i, j)))
                } else {
                    ((i, j), i2j2)
                };

                if let Some((i, j)) = i2j2 {
                    pressed2.set(Some((i, j)));
                    let start_i = usize::min(i1j1.0, i);
                    let end_i = usize::max(i1j1.0, i);

                    let min_j = usize::min(i1j1.1, j);
                    let max_j = usize::max(i1j1.1, j);

                    chosen_range.set(Some((start_i, end_i, min_j, max_j)));

                    if (start_i..=end_i).all(|i| map[i] > max_j) {
                        chosen_state.set("green");
                        clone_all![
                            chosen_range,
                            curr_stage,
                            curr_player,
                            pressed1,
                            pressed2,
                            map,
                            last_remove_line,
                            winner,
                            onend,
                        ];
                        Timeout::new(500, move || {
                            chosen_range.set(None);
                            pressed1.set(None);
                            pressed2.set(None);

                            let new_map: Vec<usize> = map
                                .iter()
                                .enumerate()
                                .map(|(idx, &v)| {
                                    if start_i <= idx && idx <= end_i {
                                        v - (max_j - min_j + 1)
                                    } else {
                                        v
                                    }
                                })
                                .collect();
                            last_remove_line.set(Some((start_i, end_i, min_j)));

                            if new_map.iter().all(|&v| v == 0) {
                                winner.set(Some(*curr_player));
                                Timeout::new(2000, move || onend.emit(*curr_player)).forget();
                            } else {
                                curr_player.set(*curr_player ^ 1);
                                curr_stage.set(Stage::Waiting);
                            }
                            map.set(new_map);
                        })
                        .forget();
                    } else {
                        chosen_state.set("red");
                        clone_all![chosen_range, curr_stage, pressed1, pressed2];
                        Timeout::new(500, move || {
                            chosen_range.set(None);
                            pressed1.set(None);
                            pressed2.set(None);
                            curr_stage.set(Stage::Waiting);
                        })
                        .forget();
                    }
                }
            },
        )
    };

    {
        clone_all![curr_stage, curr_player, players, map, calc, onclick,];
        use_effect_with(*curr_stage, move |&stage| {
            if players[*curr_player as usize] == Role::AI && stage == Stage::Waiting {
                let (i1, i2, j1, j2) = calc.borrow_mut().play(&map, 1.0);
                log!(i1, i2, j1, j2);

                Timeout::new(1000, move || onclick.emit((i1, j1, Some((i2, j2))))).forget();
            };
        });
    }

    html! {
        <div class="game-container">
            <h1>
                {
                    format!(
                        "Player {} ({})",
                        *curr_player + 1, players[*curr_player as usize].intro()
                    )
                }
            </h1>
            if let Some(winner) = winner.as_ref() {
                <div class="result-box">
                    { format!("Player {} wins!", winner + 1) }
                </div>
                <div></div>
            } else {
            <div class="map">
                {
                    (0..calc.borrow().max_height).map(|j| {
                        html! {
                            <div class="row">
                                {
                                    map.iter().enumerate().map(|(i, &v)| {
                                        html! {
                                            <div
                                                class={classes!(
                                                    "box-wrapper",
                                                    chosen_range.as_ref().and_then(|&(i1, i2, j1, j2)| {
                                                        (i1 <= i && i <= i2 && j1 <= j && j <= j2)
                                                            .then_some(*chosen_state)
                                                    }),
                                                )}
                                            >
                                                if j < v {
                                                    <div
                                                        class={classes!(
                                                            "box",
                                                            ([*pressed1, *pressed2].contains(&Some((i, j))))
                                                                .then_some("pressed"),
                                                            last_remove_line.as_ref().and_then(|&(start_i, end_i, min_j)| {
                                                                (start_i <=i && i <= end_i && j >= min_j).then_some("dropping")
                                                            }),
                                                        )}
                                                        onclick={onclick.reform(move |_| (i, j, None))}
                                                    />
                                                }
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            }
        </div>
    }
}
