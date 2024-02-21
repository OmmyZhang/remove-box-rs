use yew::{function_component, html, Callback, Html};
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Cards)]
pub fn cards(
    n_hint: u32,
    n_undo: u32,
    no_hint: bool,
    use_hint: Callback<(), Option<()>>,
    use_undo: Callback<(), Option<()>>,
) -> Html {
    html! {
        <div class="cards-box">
            <div class="card-wrapper">
                <div
                    class="card"
                    onclick={
                        Callback::from(move |_| {
                            if n_hint > 0 && !no_hint {
                                use_hint.emit(());
                            }
                        })
                    }
                >
                    { if no_hint { "😭" } else { "💡" } }
                </div>
                <span>{ format!("× {}", n_hint) }</span>
            </div>
            <div class="card-wrapper">
                <div
                    class="card"
                    onclick={
                        Callback::from(move |_| {
                            if n_undo > 0 {
                                use_undo.emit(());
                            }
                        })
                    }
                >
                    { "↩️" }
                </div>
                <span>{ format!("× {}", n_undo) }</span>
            </div>
        </div>
    }
}
