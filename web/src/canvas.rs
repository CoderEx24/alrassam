use yew::prelude::*;
use program_core::{Drawable, Line};
use super::AppState;

#[derive(PartialEq, Properties)]
pub struct CanvasProps {
    pub appstate: UseStateHandle<AppState>,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let appstate = props.appstate.clone(); 

    html! {
        <>
            <h1>{ format!("appstate.drawables().len() = {}", (*appstate).drawables().len()) }</h1>
            <p>
                {
                    (*appstate).drawables().iter().map(|drawable: &Drawable| match drawable {
                        Drawable::Line(line) => html! { line.start().x().to_string() },
                        _ => html! {}
                    }).collect::<Html>()
                }
            </p>

            <svg width="1200" height="800">
                {
                    (*appstate).drawables().iter().map(|drawable: &Drawable| match drawable {
                        Drawable::Line(line) => 
                            html! { <line 
                                x1={line.start().x().to_string()}
                                y1={line.start().y().to_string()}
                                x2={line.end().x().to_string()}
                                y2={line.end().y().to_string()}
                                style="stroke:rgb(255, 0, 0)"
                            /> },
                        _ => html! { "" }

                    }).collect::<Html>()
                }
            </svg>

        </>
    }
}


