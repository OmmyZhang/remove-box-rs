use std::rc::Rc;

use yew::{function_component, html, Html};
use yew_autoprops::autoprops;

use crate::types::Record;

#[autoprops]
#[function_component(Board)]
pub fn board(record_list: Rc<Option<Vec<Record>>>) -> Html {
    html! {
        <div class="board-wrapper">
            if let Some(list) = record_list.as_ref() {
                <table class="record-table">
                    <tbody>
                        {
                            list.iter().enumerate().map(|(idx, record)| {
                                html! {
                                    <tr>
                                        <th scope="row">{ "#" } { idx + 1 }</th>
                                        <td class="name">{ &record.name }</td>
                                        <td>{ record.score }</td>
                                        <td>
                                            {
                                                record.time.unwrap().format("%Y-%m-%d").to_string()
                                            }
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
            } else {
                <div class="lds-ring">
                    <div/>
                    <div/>
                    <div/>
                    <div/>
                </div>
            }
        </div>
    }
}
