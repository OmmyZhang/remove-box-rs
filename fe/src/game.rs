use std::cell::RefCell;
use std::rc::Rc;

use gloo_timers::callback::Timeout;
use yew::{
    classes, function_component, html, use_effect_with, use_mut_ref, use_state, Callback, Html,
};
use yew_autoprops::autoprops;

use crate::calc::Calc;
use crate::cards::Cards;
use crate::clone_all;
use crate::types::{Mode, Role, Stage};

#[autoprops]
#[function_component(Game)]
pub fn game(
    players: [Role; 2],
    init_map: Rc<Vec<usize>>,
    calc: Rc<RefCell<Calc>>,
    onend: Callback<u8>,
    mode: Mode,
    level: usize,
    n_hint: u32,
    n_undo: u32,
    use_hint: Callback<()>,
    use_undo: Callback<()>,
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

    let no_hint = use_state(|| false);

    let history = use_state(Vec::new);

    let onend_timeout = use_mut_ref(|| None);

    // 点击格子
    let onclick = {
        clone_all![
            curr_player,
            curr_stage,
            map,
            pressed1,
            pressed2,
            chosen_range,
            chosen_state,
            last_remove_line,
            winner,
            no_hint,
            history,
        ];
        Callback::from(
            move |(i, j, i2j2): (usize, usize, Option<(usize, usize)>)| {
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
                    curr_stage.set(Stage::Press2);
                    let start_i = usize::min(i1j1.0, i);
                    let end_i = usize::max(i1j1.0, i);

                    let min_j = usize::min(i1j1.1, j);
                    let max_j = usize::max(i1j1.1, j);

                    chosen_range.set(Some((start_i, end_i, min_j, max_j)));

                    if (start_i..=end_i).all(|i| map[i] > max_j) {
                        chosen_state.set(if *curr_player == 0 {
                            "success1"
                        } else {
                            "success2"
                        });
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
                            no_hint,
                            history,
                            onend_timeout,
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
                                // 如果整个game销毁，这个timeout需要被取消
                                *onend_timeout.borrow_mut() =
                                    Some(Timeout::new(1500, move || onend.emit(*curr_player)));
                            } else {
                                curr_player.set(*curr_player ^ 1);
                                curr_stage.set(Stage::Waiting);
                            }
                            map.set(new_map);
                            let mut new_history = (*history).clone();
                            new_history.push((start_i, end_i, max_j - min_j + 1));
                            history.set(new_history);
                            no_hint.set(false);
                        })
                        .forget();
                    } else {
                        chosen_state.set("failed");
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
        clone_all![curr_stage, curr_player, map, calc, onclick, level];
        use_effect_with(*curr_stage, move |&stage| {
            if players[*curr_player as usize] == Role::AI && stage == Stage::Waiting {
                let (i1, i2, j1, j2) = calc
                    .borrow_mut()
                    .play(
                        &map,
                        (level + 10 - 10.min(map.iter().sum::<usize>())) as f64 / 10.0,
                    )
                    .0;

                Timeout::new(1000, move || onclick.emit((i1, j1, Some((i2, j2))))).forget();
            };
        });
    }

    let use_hint = {
        clone_all![
            map,
            curr_player,
            curr_stage,
            calc,
            chosen_range,
            chosen_state,
            no_hint
        ];
        use_hint.filter_reform(move |()| {
            ((*curr_stage == Stage::Waiting || *curr_stage == Stage::Press1)
                && players[*curr_player as usize] == Role::Local)
                .then(|| {
                    let (play, can_win) = calc.borrow_mut().play(&map, 1.0);
                    if can_win {
                        chosen_range.set(Some(play));
                        chosen_state.set("hint");
                    } else {
                        no_hint.set(true);
                    }
                })
        })
    };

    let use_undo = {
        clone_all![
            map,
            curr_player,
            curr_stage,
            no_hint,
            history,
            pressed1,
            pressed2,
            last_remove_line,
            chosen_range,
        ];
        use_undo.filter_reform(move |()| {
            // 注意不能是then_some
            // 注意可能AI先手，len是1
            (history.len() >= 2 && players[*curr_player as usize] == Role::Local).then(|| {
                curr_stage.set(Stage::Waiting);
                pressed1.set(None);
                pressed2.set(None);
                last_remove_line.set(None);
                chosen_range.set(None);
                no_hint.set(false);

                let mut new_history = (*history).clone();
                let mut new_map = (*map).clone();
                for _ in 0..2 {
                    let (i1, i2, j) = new_history.pop().unwrap();
                    for it in new_map.iter_mut().take(i2 + 1).skip(i1) {
                        *it += j;
                    }
                }
                history.set(new_history);
                map.set(new_map);
            })
        })
    };

    html! {
        <div class="game-container">
            if mode == Mode::Pve {
                <div class="level">{ format!("Level {}", level) }</div>
            }
            <h1>
                {
                    if let Some(winner) = winner.as_ref() {
                        format!("Player {} wins!", winner + 1)
                    } else {
                        format!(
                            "Player {} ({})",
                            *curr_player + 1, players[*curr_player as usize].intro()
                        )
                    }
                }
            </h1>
            <div class="game-main">
                if mode == Mode::Pve {
                    <Cards {n_hint} {n_undo} no_hint={*no_hint} {use_hint} {use_undo} />
                }
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
                                                            .then_some(["blink", *chosen_state])
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
            </div>
        </div>
    }
}
