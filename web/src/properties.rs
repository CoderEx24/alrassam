use program_core::{ Props, CircleProps, LineProps, RectProps };
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct LineProperties {
    pub line: LineProps,
}

#[derive(Properties, PartialEq)]
struct PropertiesPanelProps {
    pub props: Props,
}

#[function_component(LinePropertiesPanel)]
fn line_properties_panel(LineProperties {
    line
}: &LineProperties) -> Html {
    html! {
        <>
            <div class={"properties"}>
                <form>
                    <label class={"properties-label"}> {"Start"} </label> <br />
                    <input class={"properties-input"} type={"text"} value={line.start.to_string()} />
                    
                    <label class={"properties-label"}> {"End"} </label> <br />
                    <input class={"properties-input"} type={"text"} value={line.end.to_string()} />

                    <label class={"properties-label"}> {"Stroke Color"} </label> <br />
                    <input class={"properties-input"} type={"text"} value={line.stroke_color.to_string()} />
                    
                    <label class={"properties-label"}> {"Stroke width"} </label> <br />
                    <input class={"properties-input"} type={"text"} value={line.stroke_width.to_string()} />
                    
                    <label class={"properties-label"}> {"fill"} </label> <br />
                    <input class={"properties-input"} type={"text"} value={line.fill.to_string()} />
                    
                </form>

            </div>
        </>

    }
}

#[function_component(PropertiesPanel)]
fn properties_panel(PropertiesPanelProps {
    props
}: &PropertiesPanelProps) -> Html {
    match props {
        Props::Line(line) => html! { <LinePropertiesPanel line={line.clone()} /> },
        _ => html! { <p>{"to be implemented :3"}</p> }
    }

}

