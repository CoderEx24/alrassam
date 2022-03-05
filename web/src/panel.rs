use yew::prelude::*;
use program_core::{Drawable, Line, Point};
use super::app_state::AppState;

#[derive(PartialEq, Properties)]
pub struct PanelProps {
    pub appstate: UseStateHandle<AppState>,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let appstate = props.appstate.clone(); 

    let add_line_onclick = {
        Callback::from(move |_| {
           let mut new_state = (*appstate).clone();
           new_state.add(&Drawable::Line(Line::new(&Point::new(1.0, 2.0), &Point::new(300.0, 3.0))));
           appstate.set(new_state);
        })
    };

    html! {
        <>
            <div>
                <button onclick={add_line_onclick}>{ "Add Line" }</button>

            </div>
        </>
    }
}


