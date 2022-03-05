use std::str;
use yew::prelude::*;
use web::{Panel, Canvas, AppState};

#[function_component(App)]
fn app() -> Html {
    let appstate = use_state(|| AppState::new());

    html! {
        <>
            <div dir="rtl">
            <h1>
                {
                    str::from_utf8(&[
                        0xd8, 0xa7, 0xd9, 0x84, 0xd8, 0xb1, 0xd8, 0xb3, 0xd8, 0xa7, 0xd9, 0x85, 0x20, 0x2d, 0x20, 0xd9, 0x86, 0xd9, 0x85, 0xd9, 0x88, 0xd8, 0xb0, 0xd8, 0xac, 0x20, 0xd8, 0xa7, 0xd9, 0x88, 0xd9, 0x84, 0xd9, 0x8a
                    ]).unwrap()
                }
            </h1>
            </div>

            <br />

            <Panel appstate={appstate.clone()} />
            <hr />
            <Canvas appstate={appstate.clone()} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
