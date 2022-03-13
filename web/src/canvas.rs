use yew::prelude::*;
use program_core::{Drawable, Line, Point};
use super::{AppState, app_state::Message};
use log;

#[derive(PartialEq, Properties)]
pub struct CanvasProps {
    pub appstate: UseStateHandle<AppState>,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let appstate = props.appstate.clone(); 
    
    let svg_onclick = Callback::from(move |evt: MouseEvent| {
        let mut new_state = (*appstate).clone();
        let click_point = Point::new(evt.x().into(), evt.y().into());
        
        if let Some(message) = new_state.current_message() {
            match message {
                Message::Line => { 
                    new_state.set_message(Some(Message::StartLine(click_point))); 
                },
                Message::StartLine(point) => {
                    let line = Line::new(&point, &click_point);
                    new_state.add(&Drawable::Line(line));
                },
                Message::FinishLine(_) => ()
            }
        }

        appstate.set(new_state);
    });

    let appstate = props.appstate.clone();

    html! {
        <>
            <h1>{ format!("appstate.drawables().len() = {}", (*appstate).drawables().len()) }</h1>
            <p>
                {
                    (*appstate).drawables().iter().map(|drawable: &Drawable| match drawable {
                        Drawable::Line(line) => html! { 
                            format!("{:?}\n", line)
                        },
                        _ => html! {}
                    }).collect::<Html>()
                }
            </p>

            <svg width="1200" height="800" style="border: 5px solid red;" onclick={svg_onclick}>
                {
                    (*appstate).drawables().iter().map(|drawable: &Drawable| match drawable {
                        Drawable::Line(line) => 
                            html! { <line 
                                x1={format!("{}px", line.start().x().to_string())}
                                y1={format!("{}px", line.start().y().to_string())}
                                x2={format!("{}px", line.end().x().to_string())}
                                y2={format!("{}px", line.end().y().to_string())}
                                style="stroke:rgb(255, 0, 0)"
                            /> },
                        _ => html! { "" }

                    }).collect::<Html>()
                }
            </svg>

        </>
    }
}


