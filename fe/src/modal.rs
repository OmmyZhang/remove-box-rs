use implicit_clone::unsync::IString;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, KeyboardEvent};
use yew::{function_component, html, use_effect_with, use_node_ref, use_state, Callback, Html};
use yew_autoprops::autoprops;

const MAX_INPUT_LEN: usize = 30;

#[autoprops]
#[function_component(Alert)]
pub fn alert(text: IString, closed_alert: Callback<()>) -> Html {
    html! {
        <div class="alert">
            <p>{ text }</p>
            <button type="button" onclick={closed_alert.reform(|_| ())}>
                <span>{ "×" }</span>
            </button>
        </div>
    }
}

#[autoprops]
#[function_component(Prompt)]
pub fn prompt(
    title: IString,
    text: IString,
    default: IString,
    placeholder: IString,
    callback: Callback<Option<String>>,
) -> Html {
    let value = use_state(|| default.clone());
    let on_cancel = callback.reform(|_| None);
    let on_submit = {
        let value = value.clone();
        let callback = callback.clone();
        Callback::from(move |_| {
            if !value.is_empty() {
                callback.emit(Some(value.to_string()));
            }
        })
    };
    let onkeypress = {
        let value = value.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key_code() == 13 && !value.is_empty() {
                callback.emit(Some(value.to_string()));
            }
        })
    };
    let onchange = {
        let value = value.clone();
        Callback::from(move |event: Event| {
            let target: EventTarget = event.target().unwrap();
            let v: String = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .chars()
                .take(MAX_INPUT_LEN)
                .collect();
            value.set(IString::from(v));
        })
    };

    let input_ref = use_node_ref();
    {
        let input_ref = input_ref.clone();
        use_effect_with(input_ref, |input_ref| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.select();
        });
    }

    html! {
        <div class="prompt-mask">
            <div class="prompt">
                <div class="head">
                    <h3>{ title }</h3>
                </div>
                <p>{ text }</p>
                <input
                    value={(*value).clone()}
                    placeholder={placeholder}
                    ref={input_ref}
                    {onchange}
                    {onkeypress}
                />
                <div class="buttons">
                    <button onclick={on_cancel}>{ "❌" }</button>
                    <button onclick={on_submit}>{ "🆗" }</button>
                </div>
            </div>
        </div>
    }
}
