use std::cell::RefCell;
use std::rc::Rc;

use gloo_console::log;
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html};
use yew_autoprops::autoprops;

use crate::calc::Calc;
use crate::types::{Role, Stage};

macro_rules! clone_all {
    [$($s:ident), * $(,)?] => {
        $(
            let $s = $s.clone();
        )*
    };
}

#[autoprops]
#[function_component(Game)]
pub fn game(players: &[Role; 2], width: usize, difficulty: f64, calc: Rc<RefCell<Calc>>) -> Html {
    let curr_player = use_state(|| 0_u8);
    let curr_stage = use_state(|| Stage::Waiting);
    let map = use_state(Vec::new);

    let pressed1 = use_state(|| None);
    let pressed2 = use_state(|| None);

    // 初始化
    {
        clone_all![curr_player, curr_stage, map, calc, pressed1, pressed2];
        use_effect_with((width, difficulty), move |(width, _)| {
            curr_player.set(0);
            curr_stage.set(Stage::Waiting);
            map.set(calc.borrow().gen_map(*width));
            pressed1.set(None);
            pressed2.set(None);
        });
    }

    // 点击格子
    let onclick = {
        clone_all![curr_player, curr_stage, map, calc, pressed1, pressed2];
        Callback::from(move |(i, j)| {
            log!(i, j);
            match *curr_stage {
                Stage::Waiting => {
                    pressed1.set(Some((i, j)));
                    curr_stage.set(Stage::Press1);
                }
                Stage::Press1 => {
                    pressed2.set(Some((i, j)));
                    curr_stage.set(Stage::Waiting);
                }
                _ => {}
            };
        })
    };

    html! {
        <div class="game-container">
            <h1> {format!("Player {}", *curr_player + 1)} </h1>
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
                                                    "box-wrapper"
                                                )}
                                            >
                                                if j <= v {
                                                    <div
                                                        class={classes!(
                                                            "box",
                                                            ([*pressed1, *pressed2].contains(&Some((i, j))))
                                                                .then_some("pressed")
                                                        )}
                                                        onclick={onclick.reform(move |_| (i, j))}
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
    }
}
